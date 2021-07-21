#[macro_export]
macro_rules! create_reasoner {
    (
        $enum_name:ident {
            $($action_name:ident),+
        }

    ) => {
        ::paste::paste!{
            #[derive(Debug,Clone,enum_default::EnumDefault,PartialEq,Eq)]
            //Create The enum component whose value is the action that scored the best
            pub enum [<$enum_name Actions>]{
                $($action_name),+
            }
            //Create The Components, a score component per action
            $(
                #[derive(Debug,Default)]
                pub struct $action_name(pub f32);
            )+
            //Create a bundle for the above created components
            #[derive(bevy::prelude::Bundle,Default)]
            pub struct [<$enum_name Actions Bundle>]{
                $(
                    pub [<$action_name:lower>]:$action_name,
                )+
                pub [<$enum_name:lower _actions>]:[<$enum_name Actions>],
            }
            //Create the "Reasoner" system for this Bundle
            pub fn [<determine_next_action_ $enum_name:lower _actions>](
                mut query:bevy::prelude::Query<( $( &$action_name, )+ &mut [<$enum_name Actions>] )>,
            ){
                for ($([<$action_name:lower>],)+mut [<$enum_name:lower _actions>] ) in query.iter_mut(){
                    let array = [

                        $(
                            (
                                [<$action_name:lower>].0 ,
                                [<$enum_name Actions>]::$action_name
                            ),
                        )+

                    ];
                    let mut index_max: usize = 0;
                    let mut temp = f32::MIN;
                    for (index, (item, _action)) in array.iter().enumerate() {
                        if *item > temp {
                            temp = *item;
                            index_max = index;
                        }
                    }
                    *[<$enum_name:lower _actions>] = array[index_max].1.clone();

                }

            }
        }

    };
}
