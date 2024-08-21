use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn trace_1_go() {
    world().run("scenarios/trace1.scen.json");
}

#[test]
fn trace_10_go() {
    world().run("scenarios/trace10.scen.json");
}

#[test]
fn trace_11_go() {
    world().run("scenarios/trace11.scen.json");
}

#[test]
fn trace_2_go() {
    world().run("scenarios/trace2.scen.json");
}

#[test]
fn trace_5_go() {
    world().run("scenarios/trace5.scen.json");
}

#[test]
fn trace_7_go() {
    world().run("scenarios/trace7.scen.json");
}

#[test]
fn trace_8_go() {
    world().run("scenarios/trace8.scen.json");
}

#[test]
fn trace_9_go() {
    world().run("scenarios/trace9.scen.json");
}
