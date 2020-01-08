use argmin::{
    prelude::*,
    solver::{gradientdescent::SteepestDescent, linesearch::MoreThuenteLineSearch},
    testfunctions::{rosenbrock_2d, rosenbrock_2d_derivative, rosenbrock_2d_hessian},
};
use serde::{Deserialize, Serialize};

/// First, create a struct for your problem
#[derive(Clone, Default, Serialize, Deserialize)]
struct Rosenbrock {
    a: f64,
    b: f64,
}

/// Implement `ArgminOp` for `Rosenbrock`
impl ArgminOp for Rosenbrock {
    /// Type of the parameter vector
    type Param = Vec<f64>;
    /// Type of the return value computed by the cost function
    type Output = f64;
    /// Type of the Hessian. Can be `()` if not needed.
    type Hessian = Vec<Vec<f64>>;
    /// Type of the Jacobian. Can be `()` if not needed.
    type Jacobian = ();

    /// Apply the cost function to a parameter `p`
    fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        Ok(rosenbrock_2d(p, self.a, self.b))
    }

    /// Compute the gradient at parameter `p`.
    fn gradient(&self, p: &Self::Param) -> Result<Self::Param, Error> {
        Ok(rosenbrock_2d_derivative(p, self.a, self.b))
    }

    /// Compute the Hessian at parameter `p`.
    fn hessian(&self, p: &Self::Param) -> Result<Self::Hessian, Error> {
        let t = rosenbrock_2d_hessian(p, self.a, self.b);
        Ok(vec![vec![t[0], t[1]], vec![t[2], t[3]]])
    }
}

fn main() {
    // Define cost function (must implement `ArgminOperator`)
    let cost = Rosenbrock { a: 1.0, b: 100.0 };

    // Define initial parameter vector
    let init_param: Vec<f64> = vec![-1.2, 1.0];

    // Set up line search
    let linesearch = MoreThuenteLineSearch::new();

    // Set up solver
    let solver = SteepestDescent::new(linesearch);

    // Run solver
    let res = Executor::new(cost, solver, init_param)
        // Add an observer which will log all iterations to the terminal
        .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
        // Set maximum iterations to 10
        .max_iters(10)
        // run the solver on the defined problem
        .run().unwrap();

    // print result
    println!("{}", res);
}
