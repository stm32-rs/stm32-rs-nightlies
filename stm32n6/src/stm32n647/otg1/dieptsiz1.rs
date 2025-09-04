///Register `DIEPTSIZ1` reader
pub type R = crate::R<DIEPTSIZ1rs>;
///Register `DIEPTSIZ1` writer
pub type W = crate::W<DIEPTSIZ1rs>;
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
        f.debug_struct("DIEPTSIZ1")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("mcnt", &self.mcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DIEPTSIZ1rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ1rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Multi count
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<DIEPTSIZ1rs> {
        MCNT_W::new(self, 29)
    }
}
/**OTG device IN endpoint 1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#OTG1:DIEPTSIZ1)*/
pub struct DIEPTSIZ1rs;
impl crate::RegisterSpec for DIEPTSIZ1rs {
    type Ux = u32;
}
///`read()` method returns [`dieptsiz1::R`](R) reader structure
impl crate::Readable for DIEPTSIZ1rs {}
///`write(|w| ..)` method takes [`dieptsiz1::W`](W) writer structure
impl crate::Writable for DIEPTSIZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTSIZ1 to value 0
impl crate::Resettable for DIEPTSIZ1rs {}
