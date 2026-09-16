///Register `HUFFBASE16` reader
pub type R = crate::R<HUFFBASE16rs>;
///Register `HUFFBASE16` writer
pub type W = crate::W<HUFFBASE16rs>;
///Field `DATA32` reader - Data 32
pub type DATA32_R = crate::FieldReader<u16>;
///Field `DATA32` writer - Data 32
pub type DATA32_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA33` reader - Data 33
pub type DATA33_R = crate::FieldReader<u16>;
///Field `DATA33` writer - Data 33
pub type DATA33_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 32
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 33
    #[inline(always)]
    pub fn data33(&self) -> DATA33_R {
        DATA33_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE16")
            .field("data32", &self.data32())
            .field("data33", &self.data33())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 32
    #[inline(always)]
    pub fn data32(&mut self) -> DATA32_W<'_, HUFFBASE16rs> {
        DATA32_W::new(self, 0)
    }
    ///Bits 16:24 - Data 33
    #[inline(always)]
    pub fn data33(&mut self) -> DATA33_W<'_, HUFFBASE16rs> {
        DATA33_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE16)*/
pub struct HUFFBASE16rs;
impl crate::RegisterSpec for HUFFBASE16rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase16::R`](R) reader structure
impl crate::Readable for HUFFBASE16rs {}
///`write(|w| ..)` method takes [`huffbase16::W`](W) writer structure
impl crate::Writable for HUFFBASE16rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE16 to value 0
impl crate::Resettable for HUFFBASE16rs {}
