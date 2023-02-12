//! # russcip
//! Safe Rust interface for SCIP.
//! 
//! # Example
//! Model and solve an integer program.
//! ```rust
//! use russcip::model::Model;
//! use russcip::model::ObjSense;
//! use russcip::status::Status;
//! use russcip::variable::VarType;
//! 
//! 
//! // Create model
//! let mut model = Model::new();
//! model.include_default_plugins();
//! model.create_prob("test");
//! model.set_obj_sense(ObjSense::Maximize);
//! model.hide_output();
//!
//! // Add variables
//! let x1 = model.add_var(0., f64::INFINITY, 3., "x1", VarType::Integer);
//! let x2 = model.add_var(0., f64::INFINITY, 4., "x2", VarType::Integer);
//!
//! // Add constraints
//! model.add_cons(&[&x1, &x2], &[2., 1.], -f64::INFINITY, 100., "c1");
//! model.add_cons(&[&x1, &x2], &[1., 2.], -f64::INFINITY, 80., "c2");
//!
//! model.solve();
//!
//! let status = model.get_status();
//! println!("Solved with status {:?}", status);
//!
//! let obj_val = model.get_obj_val();
//! println!("Objective value: {}", obj_val);
//!
//! let sol = model.get_best_sol();
//! let vars = model.get_vars();
//!
//! for var in vars {
//!     println!("{} = {}", &var.get_name(), sol.get_var_val(&var));
//! }


use scip_sys as c_api;
pub mod model;
pub mod variable;
pub mod constraint;
pub mod status;
pub mod solution;
pub mod retcode;

#[macro_export]
macro_rules! scip_call {
    ($res:expr) => {
        let res = unsafe { $res };
        if res != c_api::SCIP_Retcode_SCIP_OKAY {
            let retcode = crate::retcode::Retcode::from_c_scip_retcode(res).expect(format!("Unknown SCIP return code {}", res).as_str());
            panic!("SCIP call failed with return code {:?}", retcode);
        }
    };
}
