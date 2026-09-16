///Register `HUFFSYMB18` reader
pub type R = crate::R<HUFFSYMB18rs>;
///Register `HUFFSYMB18` writer
pub type W = crate::W<HUFFSYMB18rs>;
///Field `DATA72` reader - Data 72
pub type DATA72_R = crate::FieldReader;
///Field `DATA72` writer - Data 72
pub type DATA72_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA73` reader - Data 73
pub type DATA73_R = crate::FieldReader;
///Field `DATA73` writer - Data 73
pub type DATA73_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA74` reader - Data 74
pub type DATA74_R = crate::FieldReader;
///Field `DATA74` writer - Data 74
pub type DATA74_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA75` reader - Data 75
pub type DATA75_R = crate::FieldReader;
///Field `DATA75` writer - Data 75
pub type DATA75_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 72
    #[inline(always)]
    pub fn data72(&self) -> DATA72_R {
        DATA72_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 73
    #[inline(always)]
    pub fn data73(&self) -> DATA73_R {
        DATA73_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 74
    #[inline(always)]
    pub fn data74(&self) -> DATA74_R {
        DATA74_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 75
    #[inline(always)]
    pub fn data75(&self) -> DATA75_R {
        DATA75_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB18")
            .field("data72", &self.data72())
            .field("data73", &self.data73())
            .field("data74", &self.data74())
            .field("data75", &self.data75())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 72
    #[inline(always)]
    pub fn data72(&mut self) -> DATA72_W<'_, HUFFSYMB18rs> {
        DATA72_W::new(self, 0)
    }
    ///Bits 8:15 - Data 73
    #[inline(always)]
    pub fn data73(&mut self) -> DATA73_W<'_, HUFFSYMB18rs> {
        DATA73_W::new(self, 8)
    }
    ///Bits 16:23 - Data 74
    #[inline(always)]
    pub fn data74(&mut self) -> DATA74_W<'_, HUFFSYMB18rs> {
        DATA74_W::new(self, 16)
    }
    ///Bits 24:31 - Data 75
    #[inline(always)]
    pub fn data75(&mut self) -> DATA75_W<'_, HUFFSYMB18rs> {
        DATA75_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB18)*/
pub struct HUFFSYMB18rs;
impl crate::RegisterSpec for HUFFSYMB18rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb18::R`](R) reader structure
impl crate::Readable for HUFFSYMB18rs {}
///`write(|w| ..)` method takes [`huffsymb18::W`](W) writer structure
impl crate::Writable for HUFFSYMB18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB18 to value 0
impl crate::Resettable for HUFFSYMB18rs {}
