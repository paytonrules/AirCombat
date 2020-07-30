use gdnative::prelude::*;

pub fn load_scene<F, T>(name: &str, mut f: F) -> Option<T>
where
    F: FnMut(TRef<PackedScene>) -> Option<T>,
{
    let scene = ResourceLoader::godot_singleton().load(name, "PackedScene", false)?;
    let scene = unsafe { scene.assume_safe() };
    let packed_scene = scene.cast::<PackedScene>()?;

    f(packed_scene)
}

pub fn create_node_from_scene<T>(name: &str) -> Option<Ref<T>>
where
    T: GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    load_scene(name, |scene| {
        scene
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .map(|node| unsafe { node.assume_unique() })
            .and_then(|node| node.cast::<T>())
            .map(|node| node.into_shared())
    })
}

pub fn get_typed_node<O, F>(name: &str, owner: &Node, mut f: F)
where
    F: FnMut(TRef<O>),
    O: GodotObject + SubClass<Node>,
{
    let node = match owner
        .get_node(name)
        .map(|node| unsafe { node.assume_safe() })
        .and_then(|node| node.cast::<O>())
    {
        Some(it) => it,
        _ => return,
    };
    f(node)
}
