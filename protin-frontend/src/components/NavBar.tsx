import Link from 'next/link';
import { Save } from 'lucide-react';

import { Button } from '@/components/ui/button';

type PageType = 'new' | 'paste';

export function NavBar({ page_type }: { page_type: PageType }) {
  return (
    <nav className='sm:items-strech container flex items-center justify-between px-2 py-1 text-xl sm:px-6 lg:px-8'>
      <div>
        <Link href='/'>
          <h1 className='font-medium text-foreground'>Protin</h1>
        </Link>
      </div>
      <div>
        <Button className='text-lg' variant='outline'>
          <Save className='w-4" mr-1 h-4' />
          {page_type == 'new' ? 'Save' : 'New'}
        </Button>
      </div>
    </nav>
  );
}
