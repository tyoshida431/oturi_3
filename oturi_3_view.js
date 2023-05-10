export function getHonList(){
   var hon_list=new Array(childCount);
   var sum_kakaku=0;
   var ju_dai=0;
   for(var i=0;i<childCount;i++){
      let kakaku_str=document.getElementById("kakaku"+i).value;
      let hanpusu_str=document.getElementById("hanpusu"+i).value;
      if(kakaku_str.indexOf(":")!=-1){
         alert((i+1)+"行目 : 数字にコロンは使用できないページです。");
         return new Array(0);
      }
      if(hanpusu_str.indexOf(":")!=-1){
         alert((i+1)+"行目 : 数字にコロンは使用できないページです。");
         return new Array(0);
      }
      kakaku_str=kakaku_str.replace(",","");
      hanpusu_str=hanpusu_str.replace(",","");
      if(isNaN(kakaku_str)){
         alert((i+1)+"行目 : 価格に数字以外の文字が入力されています。");
         return new Array(0);
      }
      if(isNaN(hanpusu_str)){
         alert((i+1)+"行目 : 頒布数に数字以外の文字が入力されています。");
         return new Array(0);
      }
      if(kakaku_str=="" && hanpusu_str!=""){
         alert((i+1)+"行目 : 価格が入力されていないです。");
         return new Array(0);
      }else if(kakaku_str!="" && hanpusu_str==""){
         alert((i+1)+"行目 : 頒布数が入力されていないです。");
         return new Array(0);
      }else if(kakaku_str!="" && hanpusu_str!=""){
         let kakaku=document.getElementById("kakaku"+i).value.toString();
         let hanpusu=document.getElementById("hanpusu"+i).value.toString();
         let kakaku_num=Number(kakaku);
         sum_kakaku+=kakaku_num;
         ju_dai+=kakaku_num%100;
         if(10000<=sum_kakaku && 0<ju_dai){
            alert("頒布価格の合計が10000円を超える場合、10円台の計算に対応しておりません。申し訳ありません。");
            return new Array(0);
         }
         if(kakaku_num%10!=0){
            alert((i+1)+"行目 : 1円台の計算に対応しておりません。申し訳ありません。");
            return new Array(0);
         }
         hon_list[i]=kakaku+":"+hanpusu;         
      }
   }
   return hon_list;
}
//export function getHanpusuValue(){
//   var hanpusu_list=new Array(childCount);   
//   for(var i=0;i<childCount;i++){
//      if(document.getElementById("hanpusu"+i).value!=""){
//         let hanpusu=document.getElementById("hanpusu"+i).value.toString();
//         hanpusu_list[i]=hanpusu;
//      }
//   }
//   return hanpusu_list;
//}

export function setResult(ju,goju,hyaku,gohyaku,sen,gosen){
   let maisuDoc = document.getElementById ("result_maisu");
   let juText;
   if(0<=ju){
      juText="完売したら、</br></br>10円玉は" + ju + "枚</br>あらかじめ必要です。</br></br>";
   }else{
      ju*=-1;
      "完売したら、</br></br>10円玉は" + ju + "枚</br>余ります。</br></br>";
   }
   let gojuText;
   if(0<=goju){
      gojuText="50円玉は" + goju + "枚</br>あらかじめ必要です。</br></br>";
   }else{
      goju*=-1;
      gojuText="50円玉は" + goju + "枚</br>余ります。</br></br>";
   }
   let hyakuText;
   if(0<=hyaku){
      hyakuText="100円玉は" + hyaku + "枚</br>あらかじめ必要です。</br></br>";
   }else{
      hyaku*=-1;
      hyakuText="100円玉は" + hyaku + "枚</br>余ります。</br></br>";
   }
   let gohyakuText;
   if(0<=gohyaku){
      gohyakuText="500円玉は" + gohyaku + "枚</br>あらかじめ必要です。</br></br>";
   }else{
      gohyaku*=-1;
      gohyakuText="500円玉は" + gohyaku + "枚</br>余ります。</br></br>";
   }  
   let senText;
   if(0<=sen){
      senText="1000円札は" + sen + "枚</br>あらかじめ必要です。</br></br>";
   }else{
      sen*=-1;
      senText="1000円札は" + sen + "枚</br>余ります。</br></br>";
   }
   let gosenText;
   if(0<=gosen){
      gosenText="5000円札は" + gosen + "枚</br>あらかじめ必要です。</br></br>";
   }else{
      gosen*=-1;
      gosenText="5000円札は" + gosen + "枚</br>余ります。</br></br>";
   }
   maisuDoc.innerHTML = juText + gojuText + hyakuText + gohyakuText + senText + gosenText;
}
