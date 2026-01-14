///Register `HUFFSYMB4` reader
pub type R = crate::R<HUFFSYMB4rs>;
///Register `HUFFSYMB4` writer
pub type W = crate::W<HUFFSYMB4rs>;
///Field `DATA16` reader - Data 16
pub type DATA16_R = crate::FieldReader;
///Field `DATA16` writer - Data 16
pub type DATA16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA17` reader - Data 17
pub type DATA17_R = crate::FieldReader;
///Field `DATA17` writer - Data 17
pub type DATA17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA18` reader - Data 18
pub type DATA18_R = crate::FieldReader;
///Field `DATA18` writer - Data 18
pub type DATA18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA19` reader - Data 19
pub type DATA19_R = crate::FieldReader;
///Field `DATA19` writer - Data 19
pub type DATA19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 16
    #[inline(always)]
    pub fn data16(&self) -> DATA16_R {
        DATA16_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 17
    #[inline(always)]
    pub fn data17(&self) -> DATA17_R {
        DATA17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 18
    #[inline(always)]
    pub fn data18(&self) -> DATA18_R {
        DATA18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 19
    #[inline(always)]
    pub fn data19(&self) -> DATA19_R {
        DATA19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB4")
            .field("data16", &self.data16())
            .field("data17", &self.data17())
            .field("data18", &self.data18())
            .field("data19", &self.data19())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 16
    #[inline(always)]
    pub fn data16(&mut self) -> DATA16_W<'_, HUFFSYMB4rs> {
        DATA16_W::new(self, 0)
    }
    ///Bits 8:15 - Data 17
    #[inline(always)]
    pub fn data17(&mut self) -> DATA17_W<'_, HUFFSYMB4rs> {
        DATA17_W::new(self, 8)
    }
    ///Bits 16:23 - Data 18
    #[inline(always)]
    pub fn data18(&mut self) -> DATA18_W<'_, HUFFSYMB4rs> {
        DATA18_W::new(self, 16)
    }
    ///Bits 24:31 - Data 19
    #[inline(always)]
    pub fn data19(&mut self) -> DATA19_W<'_, HUFFSYMB4rs> {
        DATA19_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB4)*/
pub struct HUFFSYMB4rs;
impl crate::RegisterSpec for HUFFSYMB4rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb4::R`](R) reader structure
impl crate::Readable for HUFFSYMB4rs {}
///`write(|w| ..)` method takes [`huffsymb4::W`](W) writer structure
impl crate::Writable for HUFFSYMB4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB4 to value 0
impl crate::Resettable for HUFFSYMB4rs {}
