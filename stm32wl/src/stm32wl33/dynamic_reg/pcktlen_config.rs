///Register `PCKTLEN_CONFIG` reader
pub type R = crate::R<PCKTLEN_CONFIGrs>;
///Register `PCKTLEN_CONFIG` writer
pub type W = crate::W<PCKTLEN_CONFIGrs>;
///Field `PCKTLEN` reader - This bit field has different meanings/usages:
pub type PCKTLEN_R = crate::FieldReader<u16>;
///Field `PCKTLEN` writer - This bit field has different meanings/usages:
pub type PCKTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - This bit field has different meanings/usages:
    #[inline(always)]
    pub fn pcktlen(&self) -> PCKTLEN_R {
        PCKTLEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCKTLEN_CONFIG")
            .field("pcktlen", &self.pcktlen())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - This bit field has different meanings/usages:
    #[inline(always)]
    pub fn pcktlen(&mut self) -> PCKTLEN_W<'_, PCKTLEN_CONFIGrs> {
        PCKTLEN_W::new(self, 0)
    }
}
/**PCKTLEN_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`pcktlen_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcktlen_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:PCKTLEN_CONFIG)*/
pub struct PCKTLEN_CONFIGrs;
impl crate::RegisterSpec for PCKTLEN_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`pcktlen_config::R`](R) reader structure
impl crate::Readable for PCKTLEN_CONFIGrs {}
///`write(|w| ..)` method takes [`pcktlen_config::W`](W) writer structure
impl crate::Writable for PCKTLEN_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCKTLEN_CONFIG to value 0x14
impl crate::Resettable for PCKTLEN_CONFIGrs {
    const RESET_VALUE: u32 = 0x14;
}
