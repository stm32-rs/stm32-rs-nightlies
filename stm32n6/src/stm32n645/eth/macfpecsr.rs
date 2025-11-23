///Register `MACFPECSR` reader
pub type R = crate::R<MACFPECSRrs>;
///Register `MACFPECSR` writer
pub type W = crate::W<MACFPECSRrs>;
///Field `EFPE` reader - Enable Tx Frame Preemption
pub type EFPE_R = crate::BitReader;
///Field `EFPE` writer - Enable Tx Frame Preemption
pub type EFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SVER` reader - Send Verify mPacket
pub type SVER_R = crate::BitReader;
///Field `SVER` writer - Send Verify mPacket
pub type SVER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRSP` reader - Send Respond mPacket
pub type SRSP_R = crate::BitReader;
///Field `SRSP` writer - Send Respond mPacket
pub type SRSP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RVER` reader - Received Verify Frame

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RVER_R = crate::BitReader;
///Field `RVER` writer - Received Verify Frame
pub type RVER_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RRSP` reader - Received Respond Frame

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RRSP_R = crate::BitReader;
///Field `RRSP` writer - Received Respond Frame
pub type RRSP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TVER` reader - Transmitted Verify Frame

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TVER_R = crate::BitReader;
///Field `TVER` writer - Transmitted Verify Frame
pub type TVER_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TRSP` reader - Transmitted Respond Frame

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TRSP_R = crate::BitReader;
///Field `TRSP` writer - Transmitted Respond Frame
pub type TRSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable Tx Frame Preemption
    #[inline(always)]
    pub fn efpe(&self) -> EFPE_R {
        EFPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Send Verify mPacket
    #[inline(always)]
    pub fn sver(&self) -> SVER_R {
        SVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Send Respond mPacket
    #[inline(always)]
    pub fn srsp(&self) -> SRSP_R {
        SRSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - Received Verify Frame
    #[inline(always)]
    pub fn rver(&self) -> RVER_R {
        RVER_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Received Respond Frame
    #[inline(always)]
    pub fn rrsp(&self) -> RRSP_R {
        RRSP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Transmitted Verify Frame
    #[inline(always)]
    pub fn tver(&self) -> TVER_R {
        TVER_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transmitted Respond Frame
    #[inline(always)]
    pub fn trsp(&self) -> TRSP_R {
        TRSP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFPECSR")
            .field("efpe", &self.efpe())
            .field("sver", &self.sver())
            .field("srsp", &self.srsp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Tx Frame Preemption
    #[inline(always)]
    pub fn efpe(&mut self) -> EFPE_W<'_, MACFPECSRrs> {
        EFPE_W::new(self, 0)
    }
    ///Bit 1 - Send Verify mPacket
    #[inline(always)]
    pub fn sver(&mut self) -> SVER_W<'_, MACFPECSRrs> {
        SVER_W::new(self, 1)
    }
    ///Bit 2 - Send Respond mPacket
    #[inline(always)]
    pub fn srsp(&mut self) -> SRSP_W<'_, MACFPECSRrs> {
        SRSP_W::new(self, 2)
    }
    ///Bit 16 - Received Verify Frame
    #[inline(always)]
    pub fn rver(&mut self) -> RVER_W<'_, MACFPECSRrs> {
        RVER_W::new(self, 16)
    }
    ///Bit 17 - Received Respond Frame
    #[inline(always)]
    pub fn rrsp(&mut self) -> RRSP_W<'_, MACFPECSRrs> {
        RRSP_W::new(self, 17)
    }
    ///Bit 18 - Transmitted Verify Frame
    #[inline(always)]
    pub fn tver(&mut self) -> TVER_W<'_, MACFPECSRrs> {
        TVER_W::new(self, 18)
    }
    ///Bit 19 - Transmitted Respond Frame
    #[inline(always)]
    pub fn trsp(&mut self) -> TRSP_W<'_, MACFPECSRrs> {
        TRSP_W::new(self, 19)
    }
}
/**FPE control and status register

You can [`read`](crate::Reg::read) this register and get [`macfpecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfpecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACFPECSR)*/
pub struct MACFPECSRrs;
impl crate::RegisterSpec for MACFPECSRrs {
    type Ux = u32;
}
///`read()` method returns [`macfpecsr::R`](R) reader structure
impl crate::Readable for MACFPECSRrs {}
///`write(|w| ..)` method takes [`macfpecsr::W`](W) writer structure
impl crate::Writable for MACFPECSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACFPECSR to value 0
impl crate::Resettable for MACFPECSRrs {}
