// Copyright 2019 Liebi Technologies.
// This file is part of Bifrost.

// Bifrost is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Bifrost is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Bifrost.  If not, see <http://www.gnu.org/licenses/>.
use metadata::{DecodeDifferent, RuntimeMetadata, RuntimeMetadataPrefixed};
use serde::{Deserialize, Serialize};

pub type NodeMetadata = Vec<Module>;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Module {
    pub name: String,
    pub calls: Vec<Call>,
    pub events: Vec<Event>,
}

impl Module {
    fn new(name: &DecodeDifferent<&'static str, std::string::String>) -> Module {
        Module {
            name: format!("{:?}", name).replace("\"", ""),
            calls: Vec::<Call>::new(),
            events: Vec::<Event>::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Call {
    pub name: String,
    pub args: Vec<Arg>,
}

impl Call {
    fn new(name: &DecodeDifferent<&'static str, std::string::String>) -> Call {
        Call {
            name: format!("{:?}", name).replace("\"", ""),
            args: Vec::<Arg>::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Event {
    pub name: String,
    // in this case the only the argument types are provided as strings
    pub args: Vec<String>,
}

impl Event {
    fn new(name: &DecodeDifferent<&'static str, std::string::String>) -> Event {
        Event { name: format!("{:?}", name).replace("\"", ""), args: Vec::<String>::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub ty: String,
}

impl Arg {
    fn new(
        name: &DecodeDifferent<&'static str, std::string::String>,
        ty: &DecodeDifferent<&'static str, std::string::String>,
    ) -> Arg {
        Arg {
            name: format!("{:?}", name).replace("\"", ""),
            ty: format!("{:?}", ty).replace("\"", ""),
        }
    }
}

pub fn parse_metadata(metadata: &RuntimeMetadataPrefixed) -> Vec<Module> {
    let mut mod_vec = Vec::<Module>::new();
    match &metadata.1 {
        RuntimeMetadata::V8(value) => {
            match &value.modules {
                DecodeDifferent::Decoded(mods) => {
                    let modules = mods;
                    for module in modules {
                        let mut _mod = Module::new(&module.name);
                        match &module.calls {
                            Some(DecodeDifferent::Decoded(calls)) => {

                                if calls.is_empty() {
                                    // indices modules does for some reason list `Some([])' as calls and is thus counted in the call enum
                                    // there might be others doing the same.
                                    _mod.calls.push(Default::default())
                                }

                                for call in calls {
                                    let mut _call = Call::new(&call.name);
                                    match &call.arguments {
                                        DecodeDifferent::Decoded(arguments) => {
                                            for arg in arguments {
                                                _call.args.push(Arg::new(&arg.name, &arg.ty));
                                            }
                                        }
                                        _ => unreachable!(
                                            "All calls have at least the 'who' argument; qed"
                                        ),
                                    }
                                    _mod.calls.push(_call);
                                }
                            }
                            _ => println!("No calls for this module"),
                        }

                        match &module.event {
                            Some(DecodeDifferent::Decoded(event)) => {
                                if event.is_empty() {
                                    // indices modules does for some reason list `Some([])' as calls and is thus counted in the call enum
                                    // there might be others doing the same.
                                    _mod.calls.push(Default::default())
                                }

                                for e in event {
                                    let mut _event = Event::new(&e.name);
                                    match &e.arguments {
                                        DecodeDifferent::Decoded(arguments) => {
                                            for arg in arguments {
                                                _event.args.push(arg.to_string());
                                            }
                                        },
                                        _ => unreachable!("All calls have at least the 'who' argument; qed"),
                                    }
                                    _mod.events.push(_event);
                                }
                            },
                            _ => println!("No calls for this module"),
                        }

                        mod_vec.push(_mod);
                    }
                }
                _ => unreachable!("There are always modules present; qed"),
            }
        }
        _ => panic!("Unsupported metadata"),
    }
    mod_vec
}