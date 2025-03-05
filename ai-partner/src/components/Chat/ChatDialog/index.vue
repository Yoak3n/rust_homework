<script lang="ts" setup>
import { ref } from 'vue'
import { NIcon } from 'naive-ui'
import {CaretForward} from '@vicons/ionicons5'
let message = ref('')

const submittMessage = () => {
  message.value = ''
}
</script>


<template>
  <div class="dialog">
    <div class="dialog-bg"></div>
    <div class="dialog-header" data-tauri-drag-region>
    </div>
    <div class="dialog-content">
        <textarea class="dialog-textarea" rows="4" cols="50" v-model="message" @keydown="(e)=>{
        if(e.key === 'Enter'){e.preventDefault();submittMessage()}
        }" >
        </textarea>
        <button class="btn">
          <n-icon>
            <CaretForward />
          </n-icon>
        </button>
    </div>
  </div>

</template>


<style>
body{
  background-color: transparent;
}

.dialog-bg{
  position: fixed;
  height: 100%;
  width: 100%;
  filter: blur(10px);
}

.dialog-header {
  padding: 10px;
  backdrop-filter: blur(10px);
  color: #ccc;
  background-color: rgba(200, 200, 200,.5);

}

.dialog-content {
  padding: 10px;
  backdrop-filter: blur(10px);
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-radius: 10px;
  background-color: rgba(77, 107, 254, .5);
  .btn{
    height: 50px;
  }
}

.dialog-textarea {
  width: 100%;
  padding: 10px;
  resize: none;
  background-color: transparent;
  color: #fff;
}

</style>