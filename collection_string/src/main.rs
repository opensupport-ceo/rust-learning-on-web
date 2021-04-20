

/*
Reference:=> https://rinthel.github.io/rust-lang-book-ko/ch08-02-strings.html

4장에서 스트링에 관한 이야기를 했습니다만, 지금은 좀 더 깊이 살펴보겠습니다. 새로운 러스트인들은 흔히들 
스트링 부분에서 막히는데 이는 세 가지 개념의 조합으로 인한 것입니다: 가능한 에러를 꼭 노출하도록 하는 
러스트의 성향, 많은 프로그래머의 예상보다 더 복잡한 데이터 구조인 스트링, 그리고 UTF-8입니다. 다른 언어들을 
사용하다 왔을 때 이 개념들의 조합이 러스트의 스트링을 어려운 것처럼 보이게 합니다.

스트링이 컬렉션 장에 있는 이유는 스트링이 바이트의 컬렉션 및 이 바이트들을 텍스트로 통역할때 유용한 기능을 제공하는 
몇몇 메소드로 구현되어 있기 때문입니다. 이번 절에서는 생성, 갱신, 값 읽기와 같은 모든 컬렉션 타입이 가지고 있는, 
String에서의 연산에 대해 이야기 해보겠습니다. 또한 String을 다른 컬렉션들과 다르게 만드는 부분, 즉 사람과 컴퓨터가 
String 데이터를 통역하는 방식 간의 차이로 인해 생기는 String 인덱싱의 복잡함을 논의해보겠습니다.



스트링이 뭔가요?
먼저 스트링이라는 용어가 정확히 무엇을 뜻하는 것인지 정의해보겠습니다. 
러스트는 핵심 언어 기능 내에서 딱 한가지 스트링 타입만 제공하는데, 이는 바로 스트링 슬라이스인 str이고, 
이것의 참조자 형태인 &str을 많이 봤죠. 4장에서는 스트링 슬라이스에 대해 얘기했고, 이는 다른 어딘가에 저장된 UTF-8로 
인코딩된 스트링 데이터의 참조자입니다. 예를 들어, 스트링 리터럴은 프로그램의 바이너리 출력물 내에 저장되어 있으며, 
그러므로 스트링 슬라이스입니다.

String 타입은 핵심 언어 기능 내에 구현된 것이 아니고 러스트의 표준 라이브러리를 통해 제공되며, 커질 수 있고, 
가변적이며, 소유권을 갖고 있고, UTF-8로 인코딩된 스트링 타입입니다. 러스트인들이 “스트링”에 대해 이야기할 때, 
그들은 보통 String과 스트링 슬라이스 &str 타입 둘 모두를 이야기한 것이지, 이들 중 하나를 뜻한 것은 아닙니다. 
이번 절은 대부분 String에 관한 것이지만, 두 타입 모두 러스트 표준 라이브러리에서 매우 많이 사용되며 String과 
스트링 슬라이스 모두 UTF-8로 인코딩되어 있습니다.

또한 러스트 표준 라이브러리는 OsString, OsStr, CString, 그리고 CStr과 같은 몇가지 다른 스트링 타입도 제공합니다. 
심지어 어떤 라이브러리 크레이트들은 스트링 데이터를 저장하기 위해 더 많은 옵션을 제공할 수 있습니다. 
*String/ *Str이라는 
작명과 유사하게, 이들은 종종 소유권이 있는 타입과 이를 빌린 변형 타입을 제공하는데, 이는 String/&str과 비슷합니다. 
이러한 스트링 타입들은, 예를 들면 다른 종류의 인코딩이 된 텍스트를 저장하거나 다른 방식으로 메모리에 저장될 수 있습니다. 
여기서는 이러한 다른 스트링 타입은 다루지 않겠습니다; 이것들을 어떻게 쓰고 어떤 경우에 적합한지에 대해 알고 싶다면 
각각의 API 문서를 확인하시기 바랍니다.

*/



fn main() {
    println!("Hello, world!");

    //비어있는 새로운 String 생성하기
    let mut s1 = String::new();


    /*
    스트링에 담아두고 시작할 초기값을 가지고 있을 것입니다. 그런 경우, to_string 메소드를 이용하는데, 
    이는 Display 트레잇이 구현된 어떤 타입이든 사용 가능하며, 스트링 리터럴도 이 트레잇을 구현하고 있습니다. 
    두 가지 예제를 보여주고 있습니다:

    */
    let data = "initial contents";
    let s2 = data.to_string();

    // the method also works on a literal directly:
    let s3 = "initial contents".to_string(); // let s4 = String::from("initial contents");
    

    //스트링에 다양한 언어로 인삿말 저장하기
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

