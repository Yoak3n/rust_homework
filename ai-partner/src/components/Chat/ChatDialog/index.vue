<template>
    <div class="dialog">
        <div class="dialog-header">
            <span>对话框标题</span>
            <button id="expandButton">展开</button>
        </div>
        <div class="dialog-content">
            <textarea id="dialogTextarea" rows="4" cols="50"></textarea>
        </div>
    </div>

</template>


<style>
.dialog {
  width: 300px;
  border: 1px solid #ccc;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: white;
  z-index: 1000;
}

.dialog-header {
  padding: 10px;
  background: #f1f1f1;
  border-bottom: 1px solid #ccc;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dialog-content {
  padding: 10px;
}

textarea {
  width: 100%;
  resize: none;
}

</style>