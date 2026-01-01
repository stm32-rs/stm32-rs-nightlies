///Register `HUFFBASE5` reader
pub type R = crate::R<HUFFBASE5rs>;
///Register `HUFFBASE5` writer
pub type W = crate::W<HUFFBASE5rs>;
///Field `DATA10` reader - Data 10
pub type DATA10_R = crate::FieldReader<u16>;
///Field `DATA10` writer - Data 10
pub type DATA10_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA11` reader - Data 11
pub type DATA11_R = crate::FieldReader<u16>;
///Field `DATA11` writer - Data 11
pub type DATA11_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 10
    #[inline(always)]
    pub fn data10(&self) -> DATA10_R {
        DATA10_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 11
    #[inline(always)]
    pub fn data11(&self) -> DATA11_R {
        DATA11_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE5")
            .field("data10", &self.data10())
            .field("data11", &self.data11())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 10
    #[inline(always)]
    pub fn data10(&mut self) -> DATA10_W<'_, HUFFBASE5rs> {
        DATA10_W::new(self, 0)
    }
    ///Bits 16:24 - Data 11
    #[inline(always)]
    pub fn data11(&mut self) -> DATA11_W<'_, HUFFBASE5rs> {
        DATA11_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE5)*/
pub struct HUFFBASE5rs;
impl crate::RegisterSpec for HUFFBASE5rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase5::R`](R) reader structure
impl crate::Readable for HUFFBASE5rs {}
///`write(|w| ..)` method takes [`huffbase5::W`](W) writer structure
impl crate::Writable for HUFFBASE5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE5 to value 0
impl crate::Resettable for HUFFBASE5rs {}
