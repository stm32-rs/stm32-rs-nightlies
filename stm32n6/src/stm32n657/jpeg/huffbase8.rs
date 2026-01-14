///Register `HUFFBASE8` reader
pub type R = crate::R<HUFFBASE8rs>;
///Register `HUFFBASE8` writer
pub type W = crate::W<HUFFBASE8rs>;
///Field `DATA16` reader - Data 16
pub type DATA16_R = crate::FieldReader<u16>;
///Field `DATA16` writer - Data 16
pub type DATA16_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA17` reader - Data 17
pub type DATA17_R = crate::FieldReader<u16>;
///Field `DATA17` writer - Data 17
pub type DATA17_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 16
    #[inline(always)]
    pub fn data16(&self) -> DATA16_R {
        DATA16_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 17
    #[inline(always)]
    pub fn data17(&self) -> DATA17_R {
        DATA17_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE8")
            .field("data16", &self.data16())
            .field("data17", &self.data17())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 16
    #[inline(always)]
    pub fn data16(&mut self) -> DATA16_W<'_, HUFFBASE8rs> {
        DATA16_W::new(self, 0)
    }
    ///Bits 16:24 - Data 17
    #[inline(always)]
    pub fn data17(&mut self) -> DATA17_W<'_, HUFFBASE8rs> {
        DATA17_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFBASE8)*/
pub struct HUFFBASE8rs;
impl crate::RegisterSpec for HUFFBASE8rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase8::R`](R) reader structure
impl crate::Readable for HUFFBASE8rs {}
///`write(|w| ..)` method takes [`huffbase8::W`](W) writer structure
impl crate::Writable for HUFFBASE8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE8 to value 0
impl crate::Resettable for HUFFBASE8rs {}
