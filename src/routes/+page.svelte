<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  // 파일 선택 및 저장 다이얼로그 임포트
  import { open, save } from '@tauri-apps/plugin-dialog';
  // 파일 시스템 쓰기 및 읽기 기능 임포트
  import { writeFile, readFile } from '@tauri-apps/plugin-fs';

  import ResizeSection from '$lib/ResizeSection.svelte';
  import CropSection from '$lib/CropSection.svelte';

  let selectedFilePath = $state(''); // 선택된 파일 경로
  let imageDataUrl = $state(''); // 표시할 이미지 데이터 URL
  let originalWidth = $state(0); // 원본 이미지 너비
  let originalHeight = $state(0); // 원본 이미지 높이

  // 파일 선택 다이얼로그 열고 파일 경로 및 이미지 데이터 설정
  async function selectImage() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Images',
          extensions: ['png', 'jpeg', 'jpg', 'bmp', 'gif'], // 지원하는 이미지 형식
        },
      ],
    });
    if (selected && typeof selected === 'string') {
      selectedFilePath = selected;
      // 선택된 이미지 파일을 읽어와 데이터 URL로 변환하여 표시
      try {
        // Tauri 파일 시스템 플러그인을 사용하여 파일 읽기
        const fileContent = await readFile(selectedFilePath);

        // Uint8Array를 Blob으로 변환 후 FileReader를 사용하여 데이터 URL 생성
        const blob = new Blob([fileContent]);
        const reader = new FileReader();
        reader.onloadend = () => {
          imageDataUrl = reader.result as string;
          // 이미지 로드 완료 후 원본 이미지 크기 저장
          const img = new Image();
          img.onload = () => {
            originalWidth = img.naturalWidth;
            originalHeight = img.naturalHeight;
          };
          img.src = imageDataUrl;
        };
        reader.readAsDataURL(blob);
      } catch (error) {
        console.error('Failed to read image file:', error);
        imageDataUrl = '';
      }
    } else {
      selectedFilePath = ''; // 사용자가 취소함
      imageDataUrl = '';
    }
  }
</script>

<main class="flex flex-col items-center justify-center p-8">
  <h1 class="text-5xl mb-8">사진 편집기</h1>

  <!-- 파일 선택 버튼 -->
  <button onclick={selectImage} class="mb-4 p-2 bg-blue-500 text-white rounded"
    >이미지 선택</button
  >
  <p class="mb-4">선택된 파일: {selectedFilePath || '없음'}</p>

  <!-- 이미지 표시 및 자르기 영역 -->
  <CropSection
    {selectedFilePath}
    {imageDataUrl}
    {originalWidth}
    {originalHeight}
  />

  <!-- 리사이징 섹션 -->
  <ResizeSection {selectedFilePath} />
</main>

<style lang="postcss">
  @reference "tailwindcss";
  /* 필요한 추가 스타일 */
</style>
