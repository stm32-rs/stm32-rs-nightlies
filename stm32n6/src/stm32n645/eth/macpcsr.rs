///Register `MACPCSR` reader
pub type R = crate::R<MACPCSRrs>;
///Register `MACPCSR` writer
pub type W = crate::W<MACPCSRrs>;
///Field `PWRDWN` reader - Power Down
pub type PWRDWN_R = crate::BitReader;
///Field `PWRDWN` writer - Power Down
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MGKPKTEN` reader - Magic Packet Enable
pub type MGKPKTEN_R = crate::BitReader;
///Field `MGKPKTEN` writer - Magic Packet Enable
pub type MGKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPKTEN` reader - Remote wake-up Packet Enable
pub type RWKPKTEN_R = crate::BitReader;
///Field `RWKPKTEN` writer - Remote wake-up Packet Enable
pub type RWKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `MGKPRCVD` reader - Magic Packet Received

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MGKPRCVD_R = crate::BitReader;
///Field `MGKPRCVD` writer - Magic Packet Received
pub type MGKPRCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPRCVD` reader - Remote wake-up Packet Received
pub type RWKPRCVD_R = crate::BitReader;
///Field `GLBLUCAST` reader - Global Unicast
pub type GLBLUCAST_R = crate::BitReader;
///Field `GLBLUCAST` writer - Global Unicast
pub type GLBLUCAST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPFE` reader - Remote wake-up Packet Forwarding Enable
pub type RWKPFE_R = crate::BitReader;
///Field `RWKPFE` writer - Remote wake-up Packet Forwarding Enable
pub type RWKPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPTR` reader - Remote wake-up FIFO Pointer
pub type RWKPTR_R = crate::FieldReader;
///Field `RWKFILTRST` reader - Remote wake-up Packet Filter Register Pointer Reset
pub type RWKFILTRST_R = crate::BitReader;
///Field `RWKFILTRST` writer - Remote wake-up Packet Filter Register Pointer Reset
pub type RWKFILTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power Down
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet Enable
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Remote wake-up Packet Enable
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Magic Packet Received
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Remote wake-up Packet Received
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Global Unicast
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Remote wake-up Packet Forwarding Enable
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 24:28 - Remote wake-up FIFO Pointer
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 31 - Remote wake-up Packet Filter Register Pointer Reset
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPCSR")
            .field("pwrdwn", &self.pwrdwn())
            .field("mgkpkten", &self.mgkpkten())
            .field("rwkpkten", &self.rwkpkten())
            .field("rwkprcvd", &self.rwkprcvd())
            .field("glblucast", &self.glblucast())
            .field("rwkpfe", &self.rwkpfe())
            .field("rwkptr", &self.rwkptr())
            .field("rwkfiltrst", &self.rwkfiltrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power Down
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<'_, MACPCSRrs> {
        PWRDWN_W::new(self, 0)
    }
    ///Bit 1 - Magic Packet Enable
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<'_, MACPCSRrs> {
        MGKPKTEN_W::new(self, 1)
    }
    ///Bit 2 - Remote wake-up Packet Enable
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<'_, MACPCSRrs> {
        RWKPKTEN_W::new(self, 2)
    }
    ///Bit 5 - Magic Packet Received
    #[inline(always)]
    pub fn mgkprcvd(&mut self) -> MGKPRCVD_W<'_, MACPCSRrs> {
        MGKPRCVD_W::new(self, 5)
    }
    ///Bit 9 - Global Unicast
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<'_, MACPCSRrs> {
        GLBLUCAST_W::new(self, 9)
    }
    ///Bit 10 - Remote wake-up Packet Forwarding Enable
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<'_, MACPCSRrs> {
        RWKPFE_W::new(self, 10)
    }
    ///Bit 31 - Remote wake-up Packet Filter Register Pointer Reset
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<'_, MACPCSRrs> {
        RWKFILTRST_W::new(self, 31)
    }
}
/**PMT control status register

You can [`read`](crate::Reg::read) this register and get [`macpcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACPCSR)*/
pub struct MACPCSRrs;
impl crate::RegisterSpec for MACPCSRrs {
    type Ux = u32;
}
///`read()` method returns [`macpcsr::R`](R) reader structure
impl crate::Readable for MACPCSRrs {}
///`write(|w| ..)` method takes [`macpcsr::W`](W) writer structure
impl crate::Writable for MACPCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPCSR to value 0
impl crate::Resettable for MACPCSRrs {}
