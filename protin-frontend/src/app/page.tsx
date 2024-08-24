import { NavBar } from '@/components/NavBar';

export default function Home() {
  return (
    <>
      <NavBar page_type='new' />
      <main className='flex h-screen flex-col items-center justify-between bg-stone-600 p-24'></main>
    </>
  );
}
