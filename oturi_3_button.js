var childCount=1;

/**
 * 入力テキストボックスを追加します。
 * @auther tukasa
 * @date 2022/11/26
 */
function addColumn(form){
   if(childCount>15){
      alert("動作が遅くなるので、新刊15種までの対応となっております。申し訳ございません。")
      return;
   }
   const inputArea=document.getElementById("inputarea");
   const br=document.createElement("br");
   const space=document.createTextNode("\u00a0 \u00a0 \u00a0");
   const input1=document.createElement("input");
   input1.setAttribute("type","text");
   input1.setAttribute("pattern","\\d\*");
   input1.setAttribute("maxlength","7");
   input1.setAttribute("size","7");
   input1.setAttribute("value","");
   input1.setAttribute("name","kakaku")
   kakakuId="kakaku"+childCount;
   input1.setAttribute("id",kakakuId);

   const input2=document.createElement("input");
   input2.setAttribute("type","text");
   input2.setAttribute("pattern","\\d\*");
   input2.setAttribute("maxlength","5");
   input2.setAttribute("size","5");
   input2.setAttribute("value","");
   input2.setAttribute("name","hanpusu")
   hanpusuId="hanpusu"+childCount;
   input2.setAttribute("id",hanpusuId);

   inputArea.appendChild(br);
   inputArea.appendChild(input1);
   inputArea.appendChild(space)
   inputArea.appendChild(input2)

   childCount++;
}

/**
 * 入力テキストボックスを削除します。
 * @auther tukasa
 * @date 2022/11/26
 */
function delColumn(form){
   if(childCount>1){
       const inputArea=document.getElementById("inputarea");
       inputArea.removeChild(inputArea.lastChild);
       inputArea.removeChild(inputArea.lastChild);
       inputArea.removeChild(inputArea.lastChild);
       inputArea.removeChild(inputArea.lastChild);
       childCount--;
   }
}

