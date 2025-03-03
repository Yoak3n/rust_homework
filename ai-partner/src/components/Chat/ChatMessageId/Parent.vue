
<template>
  <div class="editor-container">
    <prose-mirror-editor 
      :content="initialContent"
      @update="handleEditorUpdate"
    />
    
    <div class="preview-container">
      <div 
        class="markdown-preview"
        v-html="renderedContent"
      ></div>
      <pre class="raw-markdown">{{ rawMarkdown }}</pre>
    </div>
  </div>
</template>

<script>
import ProseMirrorEditor from './ProseMirrorEditor.vue';
import { parseMarkdown } from '../utils/markdown-parser';

export default {
  components: { ProseMirrorEditor },
  data() {
    return {
      initialContent: require('prosemirror-markdown').defaultMarkdownParser.parse(
        '# 初始内容\n\n开始编辑...'
      ),
      rawMarkdown: '',
      renderedContent: ''
    };
  },
  methods: {
    handleEditorUpdate(markdown) {
      this.rawMarkdown = markdown;
      this.renderedContent = parseMarkdown(markdown);
    }
  }
};
</script>

<style>
.editor-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.markdown-preview {
  padding: 1rem;
  border: 1px solid #eee;
  border-radius: 4px;
  min-height: 300px;
  overflow: auto;
}

.raw-markdown {
  margin-top: 1rem;
  padding: 1rem;
  background: #f8f8f8;
  border-radius: 4px;
  white-space: pre-wrap;
}

.markdown-table {
  width: 100%;
  border-collapse: collapse;
  margin: 1rem 0;
}

.markdown-table td, .markdown-table th {
  border: 1px solid #ddd;
  padding: 8px;
}

.hljs {
  padding: 1rem;
  border-radius: 4px;
  margin: 1rem 0;
}
</style>