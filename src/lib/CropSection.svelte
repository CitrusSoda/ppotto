<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeFile } from '@tauri-apps/plugin-fs';

  // Props로 받을 데이터
  const { selectedFilePath, imageDataUrl, originalWidth, originalHeight } =
    $props<{
      selectedFilePath: string;
      imageDataUrl: string;
      originalWidth: number;
      originalHeight: number;
    }>();

  // 이미지 자르기 기능 상태
  let imgElement: HTMLImageElement; // 이미지 요소 참조

  let isDragging = $state(false); // 드래그 중인지 여부

  let startX = $state(0); // 드래그 시작 X 좌표
  let startY = $state(0); // 드래그 시작 Y 좌표

  let currentX = $state(0); // 현재 드래그 중인 X 좌표
  let currentY = $state(0); // 현재 드래그 중인 Y 좌표

  let cropResultMsg = $state(''); // 자르기 결과 메시지

  // 선택 영역 계산
  let selection = $derived({
    x: Math.min(startX, currentX),
    y: Math.min(startY, currentY),
    width: Math.abs(startX - currentX),
    height: Math.abs(startY - currentY),
  });

  // 이미지 자르기 커맨드 호출 및 결과 저장
  async function cropImage() {
    if (!selectedFilePath || selection.width <= 0 || selection.height <= 0) {
      cropResultMsg = '파일을 선택하고 자를 영역을 드래그하세요.';
      return;
    }
    try {
      // 표시되는 이미지의 현재 크기 가져오기
      const rect = imgElement.getBoundingClientRect();
      const displayedWidth = rect.width;
      const displayedHeight = rect.height;

      // 원본 이미지 크기 대비 표시된 이미지 크기의 스케일링 비율 계산
      const scaleX = originalWidth / displayedWidth;
      const scaleY = originalHeight / displayedHeight;

      // 드래그 좌표와 크기를 원본 이미지 크기 기준으로 스케일링 및 반올림
      const scaledX = Math.round(selection.x * scaleX);
      const scaledY = Math.round(selection.y * scaleY);
      const scaledWidth = Math.round(selection.width * scaleX);
      const scaledHeight = Math.round(selection.height * scaleY);

      // Rust 커맨드 호출하여 잘라낸 이미지 데이터 가져오기
      const imageData: number[] = await invoke('crop_image', {
        path: selectedFilePath,
        x: scaledX,
        y: scaledY,
        width: scaledWidth,
        height: scaledHeight,
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
        defaultPath: selectedFilePath.replace(/\.[^/.]+$/, '') + '_cropped.png', // 기본 파일 이름 제안
      });

      if (savePath) {
        // 선택된 경로에 바이너리 데이터 쓰기
        await writeFile(savePath, uint8Array);
        cropResultMsg = `잘라낸 이미지가 다음 경로에 저장되었습니다: ${savePath}`;
      } else {
        cropResultMsg = '저장 작업이 취소되었습니다.';
      }
    } catch (error) {
      cropResultMsg = `이미지 자르기 중 오류 발생: ${error}`;
    }
  }

  // 드래그 시작 핸들러
  function handleMouseDown(event: MouseEvent) {
    isDragging = true;

    const imgElement = event.target as HTMLImageElement;
    const rect = imgElement.getBoundingClientRect();

    startX = Math.round(event.clientX - rect.left);
    startY = Math.round(event.clientY - rect.top);

    currentX = startX;
    currentY = startY;

    window.addEventListener('mouseup', handleMouseUp);

    event.preventDefault(); // 기본 드래그 동작 방지
  }

  // 드래그 중 핸들러
  function handleMouseMove(event: MouseEvent) {
    if (!isDragging) return;
    const imgElement = event.target as HTMLImageElement;
    const rect = imgElement.getBoundingClientRect();

    currentX = Math.round(event.clientX - rect.left);
    currentY = Math.round(event.clientY - rect.top);
  }

  // 드래그 종료 핸들러
  function handleMouseUp() {
    window.removeEventListener('mouseup', handleMouseUp);
    isDragging = false;
  }

  // 자르기 영역 초기화 함수
  function resetCropSelection() {
    startX = 0;
    startY = 0;
    currentX = 0;
    currentY = 0;
  }
</script>

<!-- 이미지 표시 및 자르기 영역 -->
{#if imageDataUrl}
  <div class="relative inline-block mb-8">
    <img
      bind:this={imgElement}
      onmousedown={handleMouseDown}
      onmousemove={handleMouseMove}
      src={imageDataUrl}
      alt="Selected image"
      class="max-w-full max-h-96 border border-gray-300"
    />
    {#if isDragging || (selection.width > 0 && selection.height > 0)}
      <div
        class="absolute border-2 border-red-500 pointer-events-none"
        style="left: {selection.x}px; top: {selection.y}px; width: {selection.width}px; height: {selection.height}px;"
      ></div>
    {/if}
  </div>
{/if}

<!-- 자르기 섹션 -->
<section class="p-4 border rounded shadow-sm w-full max-w-md">
  <h2 class="text-2xl mb-4 text-center">이미지 자르기 (영역 선택)</h2>
  <div class="flex flex-col items-center space-y-4">
    <p>
      선택 영역: X:{selection.x}, Y:{selection.y}, 너비:{selection.width}, 높이:{selection.height}
    </p>
    <button onclick={cropImage} class="p-2 bg-yellow-500 text-white rounded"
      >자르기 실행</button
    >
    <button
      onclick={resetCropSelection}
      class="p-2 bg-gray-500 text-white rounded">선택 영역 초기화</button
    >
    {#if cropResultMsg}
      <p class="text-sm text-gray-600 mt-2">{cropResultMsg}</p>
    {/if}
  </div>
</section>
