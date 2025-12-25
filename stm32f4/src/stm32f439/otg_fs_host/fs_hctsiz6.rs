///Register `FS_HCTSIZ6` reader
pub type R = crate::R<FS_HCTSIZ6rs>;
///Register `FS_HCTSIZ6` writer
pub type W = crate::W<FS_HCTSIZ6rs>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HCTSIZ6")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("dpid", &self.dpid())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, FS_HCTSIZ6rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, FS_HCTSIZ6rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Data PID
    #[inline(always)]
    pub fn dpid(&mut self) -> DPID_W<'_, FS_HCTSIZ6rs> {
        DPID_W::new(self, 29)
    }
}
/**OTG_FS host channel-6 transfer size register

You can [`read`](crate::Reg::read) this register and get [`fs_hctsiz6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hctsiz6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_FS_HOST:FS_HCTSIZ6)*/
pub struct FS_HCTSIZ6rs;
impl crate::RegisterSpec for FS_HCTSIZ6rs {
    type Ux = u32;
}
///`read()` method returns [`fs_hctsiz6::R`](R) reader structure
impl crate::Readable for FS_HCTSIZ6rs {}
///`write(|w| ..)` method takes [`fs_hctsiz6::W`](W) writer structure
impl crate::Writable for FS_HCTSIZ6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_HCTSIZ6 to value 0
impl crate::Resettable for FS_HCTSIZ6rs {}
