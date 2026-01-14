///Register `HUFFSYMB78` reader
pub type R = crate::R<HUFFSYMB78rs>;
///Register `HUFFSYMB78` writer
pub type W = crate::W<HUFFSYMB78rs>;
///Field `DATA312` reader - Data 312
pub type DATA312_R = crate::FieldReader;
///Field `DATA312` writer - Data 312
pub type DATA312_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA313` reader - Data 313
pub type DATA313_R = crate::FieldReader;
///Field `DATA313` writer - Data 313
pub type DATA313_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA314` reader - Data 314
pub type DATA314_R = crate::FieldReader;
///Field `DATA314` writer - Data 314
pub type DATA314_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA315` reader - Data 315
pub type DATA315_R = crate::FieldReader;
///Field `DATA315` writer - Data 315
pub type DATA315_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 312
    #[inline(always)]
    pub fn data312(&self) -> DATA312_R {
        DATA312_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 313
    #[inline(always)]
    pub fn data313(&self) -> DATA313_R {
        DATA313_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 314
    #[inline(always)]
    pub fn data314(&self) -> DATA314_R {
        DATA314_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 315
    #[inline(always)]
    pub fn data315(&self) -> DATA315_R {
        DATA315_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB78")
            .field("data312", &self.data312())
            .field("data313", &self.data313())
            .field("data314", &self.data314())
            .field("data315", &self.data315())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 312
    #[inline(always)]
    pub fn data312(&mut self) -> DATA312_W<'_, HUFFSYMB78rs> {
        DATA312_W::new(self, 0)
    }
    ///Bits 8:15 - Data 313
    #[inline(always)]
    pub fn data313(&mut self) -> DATA313_W<'_, HUFFSYMB78rs> {
        DATA313_W::new(self, 8)
    }
    ///Bits 16:23 - Data 314
    #[inline(always)]
    pub fn data314(&mut self) -> DATA314_W<'_, HUFFSYMB78rs> {
        DATA314_W::new(self, 16)
    }
    ///Bits 24:31 - Data 315
    #[inline(always)]
    pub fn data315(&mut self) -> DATA315_W<'_, HUFFSYMB78rs> {
        DATA315_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB78)*/
pub struct HUFFSYMB78rs;
impl crate::RegisterSpec for HUFFSYMB78rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb78::R`](R) reader structure
impl crate::Readable for HUFFSYMB78rs {}
///`write(|w| ..)` method takes [`huffsymb78::W`](W) writer structure
impl crate::Writable for HUFFSYMB78rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB78 to value 0
impl crate::Resettable for HUFFSYMB78rs {}
