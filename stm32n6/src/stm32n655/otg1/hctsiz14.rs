///Register `HCTSIZ14` reader
pub type R = crate::R<HCTSIZ14rs>;
///Register `HCTSIZ14` writer
pub type W = crate::W<HCTSIZ14rs>;
///Field `XFRSIZ` reader - Transfer size
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - Transfer size
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - Packet count
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - Packet count
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `DPID` reader - Data PID
pub type DPID_R = crate::FieldReader;
///Field `DPID` writer - Data PID
pub type DPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DOPNG` reader - Do Ping
pub type DOPNG_R = crate::BitReader;
///Field `DOPNG` writer - Do Ping
pub type DOPNG_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bits 29:30 - Data PID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - Do Ping
    #[inline(always)]
    pub fn dopng(&self) -> DOPNG_R {
        DOPNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ14")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("dpid", &self.dpid())
            .field("dopng", &self.dopng())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<HCTSIZ14rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<HCTSIZ14rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Data PID
    #[inline(always)]
    pub fn dpid(&mut self) -> DPID_W<HCTSIZ14rs> {
        DPID_W::new(self, 29)
    }
    ///Bit 31 - Do Ping
    #[inline(always)]
    pub fn dopng(&mut self) -> DOPNG_W<HCTSIZ14rs> {
        DOPNG_W::new(self, 31)
    }
}
/**OTG host channel 14 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:HCTSIZ14)*/
pub struct HCTSIZ14rs;
impl crate::RegisterSpec for HCTSIZ14rs {
    type Ux = u32;
}
///`read()` method returns [`hctsiz14::R`](R) reader structure
impl crate::Readable for HCTSIZ14rs {}
///`write(|w| ..)` method takes [`hctsiz14::W`](W) writer structure
impl crate::Writable for HCTSIZ14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCTSIZ14 to value 0
impl crate::Resettable for HCTSIZ14rs {}
