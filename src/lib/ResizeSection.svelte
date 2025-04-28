<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeFile } from '@tauri-apps/plugin-fs';

  // Props로 받을 파일 경로
  const { selectedFilePath } = $props<{ selectedFilePath: string }>();

  // 리사이징 기능 상태
  let newWidth = $state(0); // 새로운 너비
  let newHeight = $state(0); // 새로운 높이
  let resizeResultMsg = $state(''); // 리사이징 결과 메시지

  // Rust 리사이즈 커맨드 호출 및 결과 저장
  async function resizeImage() {
    if (!selectedFilePath || newWidth <= 0 || newHeight <= 0) {
      resizeResultMsg = '파일을 선택하고 유효한 크기를 입력하세요.';
      return;
    }
    try {
      // Rust 커맨드 호출하여 리사이즈된 이미지 데이터 가져오기 (Vec<u8>는 number[]로 변환됨)
      const imageData: number[] = await invoke('resize_image', {
        path: selectedFilePath,
        width: newWidth,
        height: newHeight,
      });

      // 저장용 Uint8Array로 변환
      const uint8Array = new Uint8Array(imageData);

      // 저장 파일 다이얼로그 열기
      const savePath = await save({
        filters: [
          {
            name: 'PNG Image',
            extensions: ['png'], // Rust는 PNG로 반환한다고 가정
          },
        ],
        defaultPath: selectedFilePath.replace(/\.[^/.]+$/, '') + '_resized.png', // 기본 파일 이름 제안
      });

      if (savePath) {
        // 선택된 경로에 바이너리 데이터 쓰기
        await writeFile(savePath, uint8Array);
        resizeResultMsg = `리사이즈된 이미지가 다음 경로에 저장되었습니다: ${savePath}`;
      } else {
        resizeResultMsg = '저장 작업이 취소되었습니다.';
      }
    } catch (error) {
      resizeResultMsg = `이미지 리사이징 중 오류 발생: ${error}`;
    }
  }
</script>

<!-- 리사이징 섹션 -->
<section class="mb-8 p-4 border rounded shadow-sm w-full max-w-md">
  <h2 class="text-2xl mb-4 text-center">이미지 리사이징 (값 입력)</h2>
  <div class="flex flex-col items-center space-y-4">
    <div class="flex space-x-4">
      <input
        type="number"
        bind:value={newWidth}
        placeholder="너비"
        min="1"
        class="border p-2 w-24 text-center"
      />
      <input
        type="number"
        bind:value={newHeight}
        placeholder="높이"
        min="1"
        class="border p-2 w-24 text-center"
      />
    </div>
    <button onclick={resizeImage} class="p-2 bg-green-500 text-white rounded"
      >리사이즈 실행</button
    >
    {#if resizeResultMsg}
      <p class="text-sm text-gray-600 mt-2">{resizeResultMsg}</p>
    {/if}
  </div>
</section>
