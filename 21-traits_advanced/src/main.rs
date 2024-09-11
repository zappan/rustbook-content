mod associated_types;
mod default_generic_type;
mod same_name_methods;
mod supertraits;
mod ext_traits_ext_types;

fn main() {
  associated_types::run();
  default_generic_type::run();
  same_name_methods::run();
  supertraits::run();
  ext_traits_ext_types::run();
}
