use rust_fsm::*;

#[derive(Clone, Copy, Debug)]
enum MyMachineTransitionEvent {
    InitMaintenance,
    WaitingForTaskAcompleted,
}

#[derive(Debug)]
struct MyMachineTransitionInput {
    desired_next_state: MyMachineTransitionEvent,
    user_is_admin: bool,
}

#[derive(Debug, PartialEq)]
struct MyMachineTransitionOutput;

#[derive(Clone, Copy, Debug, PartialEq)]
enum MyMachineStates {
    Active,
    CreateTaskA,
    WaitingForTaskACompleted,
}

#[derive(Debug)]
struct MyMachine;

impl StateMachineImpl for MyMachine {
    type Input = MyMachineTransitionInput;
    type State = MyMachineStates;
    type Output = MyMachineTransitionOutput;
    const INITIAL_STATE: Self::State = MyMachineStates::Active;

    fn transition(state: &Self::State, input: &Self::Input) -> Option<Self::State> {
        match (state, input.desired_next_state) {
            (MyMachineStates::Active, MyMachineTransitionEvent::InitMaintenance) => {
                if input.user_is_admin {
                    Some(MyMachineStates::CreateTaskA)
                } else {
                    None
                }
            }
            (
                MyMachineStates::CreateTaskA,
                MyMachineTransitionEvent::WaitingForTaskAcompleted,
            ) => Some(MyMachineStates::WaitingForTaskACompleted),
            _ => None,
        }
    }

    fn output(_state: &Self::State, _input: &Self::Input) -> Option<Self::Output> {
        None
    }
}

#[cfg(test)]
mod tests {
    use rust_fsm::StateMachine;

    use crate::*;

    #[test]
    fn admin_can_transit_from_active_to_createtaska() {
        let mut machine: StateMachine<MyMachine> = StateMachine::new();
        let i: MyMachineTransitionInput = MyMachineTransitionInput {
            desired_next_state: MyMachineTransitionEvent::InitMaintenance,
            user_is_admin: true,
        };
        let res = machine.consume(&i).unwrap();
        assert_eq!(res, None);
        assert_eq!(machine.state(), &MyMachineStates::CreateTaskA);
    }

    #[test]
    fn user_cannot_transit_from_active_to_createtaska() {
        let mut machine: StateMachine<MyMachine> = StateMachine::new();
        let i: MyMachineTransitionInput = MyMachineTransitionInput {
            desired_next_state: MyMachineTransitionEvent::InitMaintenance,
            user_is_admin: false,
        };
        let res = machine.consume(&i);
        assert!(res.is_err(), "Transition not allowed");
    }

    #[test]
    fn admin_cannot_transit_from_active_to_waitingfortaskacompleted() {
        let mut machine: StateMachine<MyMachine> = StateMachine::new();
        let i: MyMachineTransitionInput = MyMachineTransitionInput {
            desired_next_state: MyMachineTransitionEvent::WaitingForTaskAcompleted,
            user_is_admin: true,
        };
        let res = machine.consume(&i);
        assert!(res.is_err(), "Transition not allowed");
    }
}
