pub mod bounding;
pub mod coord;
pub mod mesh;
pub mod texture;
pub mod transform;
pub mod rectangle;

///////////////////////////////////////
///                                 ///
///    没完成 要大改  确保可拓展性    ///
///                                 ///
///////////////////////////////////////

#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_os_win10")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
pub mod env {
    use crate::{
        manager::{
            self,
            datum::{self, env::Datum},
            execute::{
                env::TaskQueue,
                template::call_back_template::{Callback0MR0R, Callback0MR10R},
            },
        },
        node::env::{NodeD, NodeT},
    };

    use super::transform::env::TransformD;

    pub enum ModelTask {
        None,
        UpdateGlobalTransform(
            usize, //dat model index
            Callback0MR0R,
        ),
        EulerRotate(),
        ResetQuaternion(),
        Dispalce(),
        Scale(),
    }

    #[repr(C, align(8))]
    pub struct ModelExeAttachment {
        index_transform_task: u64,
    }

    #[derive(Default)]
    #[repr(C, align(8))]
    pub struct ModelE {
        pub attachment: ModelExeAttachment,
        pub id:u64,
    }

    impl ModelE {
        pub fn set_id(&mut self , id_in:u64){
            self.id = id_in;
        }
        pub fn id_mut(&mut self) -> &mut u64{
            return &mut self.id;
        }
        pub fn build() -> Self {
            return Self {
                attachment: Default::default(),
                id: 0,
            };
        }

        pub fn link_task_queue(&mut self, tqin: &mut Datum<TaskQueue<ModelTask>>) {
            tqin.alloc_data(TaskQueue::default(), Some(self.attachment.index_transform_task));
        }

        pub fn update_global_transform_sync(
            datum_trans: &mut Datum<TransformD>,
            datum_model: &mut Datum<ModelD>,
        ) {
            let mut _node_index_array = Vec::<u64>::default();
            let mut _parent_count: usize = 0;
            // check if need change
            for mi in datum_model.iter_mut() {
                if mi.as_mut().unwrap().attachment.is_transform_update {
                    mi.as_mut().unwrap().attachment.is_transform_update = false;
                    _node_index_array.push(mi.as_mut().unwrap().node_ref().index_self);

                    _parent_count = _parent_count + 1;
                }
            }
            if !_node_index_array.is_empty() {
                for i in 0.._parent_count {
                    _node_index_array.extend(ModelD::find_all_subnode_index(datum_model, i as u64).unwrap())
                }
                for i in _parent_count.._node_index_array.len() {
                    TransformD::update_global(
                        datum_trans,
                        datum_model.vec_ref()[_node_index_array[i] as usize]
                            .as_ref()
                            .unwrap()
                            .node_ref()
                            .index_parent,
                        _node_index_array[i],
                    )
                }
            }
        }

        pub fn euler_rotate() {}
        
        pub fn quat_rotate() {}

        pub fn dispalce() {}

        pub fn scale() {}

        pub fn exe_transform() {}
    }

    #[repr(C, align(8))]
    pub struct ModelAttachment {
        index_mesh: u64,
        index_transform: u64,
        index_textures: Vec<u64>,
        is_transform_update: bool,
        is_active: bool,
    }

    impl Default for ModelAttachment {
        fn default() -> Self {
            Self {
                index_mesh: u64::MAX,
                index_transform: u64::MAX,
                index_textures: Default::default(),
                is_active: true,
                is_transform_update: true,
            }
        }
    }

    #[repr(C, align(8))]
    pub struct ModelD {
        //node_option
        attachment: ModelAttachment,
        id: u64,
        node: NodeD,
    }

    impl ModelD {
        pub fn build() -> Self {
            Self {
                id: u64::MAX,
                node: Default::default(),
                attachment: Default::default(),
            }
        }

        pub fn build_from_meta(mut self) -> Self {
            return self;
        }

        pub fn set_transform(&mut self, index_in: u64) {
            self.attachment.index_transform = index_in;
        }
        pub fn set_mesh(&mut self, index_in: u64) {
            self.attachment.index_mesh = index_in;
        }
        pub fn set_tex(&mut self, index_in: u64) {
            self.attachment.index_textures.push(index_in) ;
        }
    }

    impl NodeT for ModelD {
        fn node_ref(self: &Self) -> &NodeD {
            return &self.node;
        }

        fn node_mut(self: &mut Self) -> &mut NodeD {
            return &mut self.node;
        }
    }

    impl Default for ModelExeAttachment {
        fn default() -> Self {
            Self {
                index_transform_task: 0,
            }
        }
    }

    impl Default for ModelTask {
        fn default() -> Self {
            return Self::None;
        }
    }

    impl Default for ModelD {
        fn default() -> Self {
            Self {
                id: u64::MAX,
                attachment: Default::default(),
                node: Default::default(),
            }
        }
    }
}
