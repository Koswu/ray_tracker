use crate::renderer::CheckIsHitObjectAble;


pub struct SceneStub{

}
impl SceneStub{
    pub fn new() -> Self{
        SceneStub {  }
    }
}

impl CheckIsHitObjectAble for SceneStub{
    fn check_is_hit_object(&self, ray: &crate::ray::ReverseRay) -> bool {
        true
    }
}
