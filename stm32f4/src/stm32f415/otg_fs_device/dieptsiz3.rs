///Register `DIEPTSIZ3` reader
pub type R = crate::R<DIEPTSIZ3rs>;
///Register `DIEPTSIZ3` writer
pub type W = crate::W<DIEPTSIZ3rs>;
///Field `XFRSIZ` reader - Transfer size
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - Transfer size
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - Packet count
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - Packet count
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `MCNT` reader - Multi count
pub type MCNT_R = crate::FieldReader;
///Field `MCNT` writer - Multi count
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - Multi count
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ3")
            .field("mcnt", &self.mcnt())
            .field("pktcnt", &self.pktcnt())
            .field("xfrsiz", &self.xfrsiz())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, DIEPTSIZ3rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DIEPTSIZ3rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Multi count
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<'_, DIEPTSIZ3rs> {
        MCNT_W::new(self, 29)
    }
}
/**device endpoint-3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_DEVICE:DIEPTSIZ3)*/
pub struct DIEPTSIZ3rs;
impl crate::RegisterSpec for DIEPTSIZ3rs {
    type Ux = u32;
}
///`read()` method returns [`dieptsiz3::R`](R) reader structure
impl crate::Readable for DIEPTSIZ3rs {}
///`write(|w| ..)` method takes [`dieptsiz3::W`](W) writer structure
impl crate::Writable for DIEPTSIZ3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTSIZ3 to value 0
impl crate::Resettable for DIEPTSIZ3rs {}
