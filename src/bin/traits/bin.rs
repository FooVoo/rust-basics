mod traits_01_Shape_Calc;
mod traits_02_Borrowing_detective;
mod traits_03_Library_system;
mod traits_04_Reference_Lifetimes;
mod traits_05_Animal_Shelter;
// mod traits_06_Data_Processor;
// mod traits_07_Trait_Bounds;
// mod traits_08_Finale_Text_Editor;

fn main() {
    println!("Hello from traits bin!");
    traits_01_Shape_Calc::problem::main();
    traits_02_Borrowing_detective::problem::main();
    traits_03_Library_system::problem::main();
    traits_04_Reference_Lifetimes::problem::main();
    traits_05_Animal_Shelter::problem::main();
}