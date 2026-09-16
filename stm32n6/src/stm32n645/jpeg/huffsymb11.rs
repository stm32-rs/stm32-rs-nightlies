///Register `HUFFSYMB11` reader
pub type R = crate::R<HUFFSYMB11rs>;
///Register `HUFFSYMB11` writer
pub type W = crate::W<HUFFSYMB11rs>;
///Field `DATA44` reader - Data 44
pub type DATA44_R = crate::FieldReader;
///Field `DATA44` writer - Data 44
pub type DATA44_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA45` reader - Data 45
pub type DATA45_R = crate::FieldReader;
///Field `DATA45` writer - Data 45
pub type DATA45_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA46` reader - Data 46
pub type DATA46_R = crate::FieldReader;
///Field `DATA46` writer - Data 46
pub type DATA46_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA47` reader - Data 47
pub type DATA47_R = crate::FieldReader;
///Field `DATA47` writer - Data 47
pub type DATA47_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 44
    #[inline(always)]
    pub fn data44(&self) -> DATA44_R {
        DATA44_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 45
    #[inline(always)]
    pub fn data45(&self) -> DATA45_R {
        DATA45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 46
    #[inline(always)]
    pub fn data46(&self) -> DATA46_R {
        DATA46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 47
    #[inline(always)]
    pub fn data47(&self) -> DATA47_R {
        DATA47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB11")
            .field("data44", &self.data44())
            .field("data45", &self.data45())
            .field("data46", &self.data46())
            .field("data47", &self.data47())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 44
    #[inline(always)]
    pub fn data44(&mut self) -> DATA44_W<'_, HUFFSYMB11rs> {
        DATA44_W::new(self, 0)
    }
    ///Bits 8:15 - Data 45
    #[inline(always)]
    pub fn data45(&mut self) -> DATA45_W<'_, HUFFSYMB11rs> {
        DATA45_W::new(self, 8)
    }
    ///Bits 16:23 - Data 46
    #[inline(always)]
    pub fn data46(&mut self) -> DATA46_W<'_, HUFFSYMB11rs> {
        DATA46_W::new(self, 16)
    }
    ///Bits 24:31 - Data 47
    #[inline(always)]
    pub fn data47(&mut self) -> DATA47_W<'_, HUFFSYMB11rs> {
        DATA47_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB11)*/
pub struct HUFFSYMB11rs;
impl crate::RegisterSpec for HUFFSYMB11rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb11::R`](R) reader structure
impl crate::Readable for HUFFSYMB11rs {}
///`write(|w| ..)` method takes [`huffsymb11::W`](W) writer structure
impl crate::Writable for HUFFSYMB11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB11 to value 0
impl crate::Resettable for HUFFSYMB11rs {}
