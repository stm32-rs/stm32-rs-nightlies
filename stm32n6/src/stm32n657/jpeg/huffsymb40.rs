///Register `HUFFSYMB40` reader
pub type R = crate::R<HUFFSYMB40rs>;
///Register `HUFFSYMB40` writer
pub type W = crate::W<HUFFSYMB40rs>;
///Field `DATA160` reader - Data 160
pub type DATA160_R = crate::FieldReader;
///Field `DATA160` writer - Data 160
pub type DATA160_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA161` reader - Data 161
pub type DATA161_R = crate::FieldReader;
///Field `DATA161` writer - Data 161
pub type DATA161_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA162` reader - Data 162
pub type DATA162_R = crate::FieldReader;
///Field `DATA162` writer - Data 162
pub type DATA162_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA163` reader - Data 163
pub type DATA163_R = crate::FieldReader;
///Field `DATA163` writer - Data 163
pub type DATA163_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 160
    #[inline(always)]
    pub fn data160(&self) -> DATA160_R {
        DATA160_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 161
    #[inline(always)]
    pub fn data161(&self) -> DATA161_R {
        DATA161_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 162
    #[inline(always)]
    pub fn data162(&self) -> DATA162_R {
        DATA162_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 163
    #[inline(always)]
    pub fn data163(&self) -> DATA163_R {
        DATA163_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB40")
            .field("data160", &self.data160())
            .field("data161", &self.data161())
            .field("data162", &self.data162())
            .field("data163", &self.data163())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 160
    #[inline(always)]
    pub fn data160(&mut self) -> DATA160_W<'_, HUFFSYMB40rs> {
        DATA160_W::new(self, 0)
    }
    ///Bits 8:15 - Data 161
    #[inline(always)]
    pub fn data161(&mut self) -> DATA161_W<'_, HUFFSYMB40rs> {
        DATA161_W::new(self, 8)
    }
    ///Bits 16:23 - Data 162
    #[inline(always)]
    pub fn data162(&mut self) -> DATA162_W<'_, HUFFSYMB40rs> {
        DATA162_W::new(self, 16)
    }
    ///Bits 24:31 - Data 163
    #[inline(always)]
    pub fn data163(&mut self) -> DATA163_W<'_, HUFFSYMB40rs> {
        DATA163_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB40)*/
pub struct HUFFSYMB40rs;
impl crate::RegisterSpec for HUFFSYMB40rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb40::R`](R) reader structure
impl crate::Readable for HUFFSYMB40rs {}
///`write(|w| ..)` method takes [`huffsymb40::W`](W) writer structure
impl crate::Writable for HUFFSYMB40rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB40 to value 0
impl crate::Resettable for HUFFSYMB40rs {}
