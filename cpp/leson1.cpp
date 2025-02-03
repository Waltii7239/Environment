#include <iostream>
#include <iterator>
#include <string>

using namespace std;

enum class animal{
  k9,
  cat
};

class pet{
  public:
    std::string name;
    int age;
    animal kind; 
  
    std::string returnName(){
      return name;
    }
  
    int returnAge(){
      return age;
    }

    void returnAnimalType(){
      switch (kind) {
        case animal::k9:
          cout << "k9" << endl;
        break;
        case animal::cat:
          cout << "cat" << endl;
        break;
      }
    }
  private:

};

int main(){

  pet mypet;
  mypet.name = "test";
  mypet.age = 23;
  mypet.kind = animal::k9;
  
  std::cout << mypet.returnName() << std::endl;
  std::cout << mypet.returnAge() << std::endl;
  mypet.returnAnimalType();

  return 0;
}
