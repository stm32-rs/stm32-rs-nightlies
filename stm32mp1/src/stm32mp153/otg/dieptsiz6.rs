///Register `DIEPTSIZ6` reader
pub type R = crate::R<DIEPTSIZ6rs>;
///Register `DIEPTSIZ6` writer
pub type W = crate::W<DIEPTSIZ6rs>;
///Field `XFRSIZ` reader - XFRSIZ
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - XFRSIZ
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - PKTCNT
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - PKTCNT
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `MCNT` reader - MCNT
pub type MCNT_R = crate::FieldReader;
///Field `MCNT` writer - MCNT
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:18 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - MCNT
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ6")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("mcnt", &self.mcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DIEPTSIZ6rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ6rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - MCNT
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<DIEPTSIZ6rs> {
        MCNT_W::new(self, 29)
    }
}
/**The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`dieptsiz6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:DIEPTSIZ6)*/
pub struct DIEPTSIZ6rs;
impl crate::RegisterSpec for DIEPTSIZ6rs {
    type Ux = u32;
}
///`read()` method returns [`dieptsiz6::R`](R) reader structure
impl crate::Readable for DIEPTSIZ6rs {}
///`write(|w| ..)` method takes [`dieptsiz6::W`](W) writer structure
impl crate::Writable for DIEPTSIZ6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTSIZ6 to value 0
impl crate::Resettable for DIEPTSIZ6rs {}
