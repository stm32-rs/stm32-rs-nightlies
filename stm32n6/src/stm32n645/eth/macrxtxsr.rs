///Register `MACRXTXSR` reader
pub type R = crate::R<MACRXTXSRrs>;
///Register `MACRXTXSR` writer
pub type W = crate::W<MACRXTXSRrs>;
/**Field `TJT` reader - Transmit Jabber Timeout

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TJT_R = crate::BitReader;
///Field `TJT` writer - Transmit Jabber Timeout
pub type TJT_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `NCARR` reader - No Carrier

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type NCARR_R = crate::BitReader;
///Field `NCARR` writer - No Carrier
pub type NCARR_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `LCARR` reader - Loss of Carrier

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type LCARR_R = crate::BitReader;
///Field `LCARR` writer - Loss of Carrier
pub type LCARR_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `EXDEF` reader - Excessive Deferral

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type EXDEF_R = crate::BitReader;
///Field `EXDEF` writer - Excessive Deferral
pub type EXDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `LCOL` reader - Late Collision

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type LCOL_R = crate::BitReader;
///Field `LCOL` writer - Late Collision
pub type LCOL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `EXCOL` reader - Excessive Collisions

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type EXCOL_R = crate::BitReader;
///Field `EXCOL` writer - Excessive Collisions
pub type EXCOL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RWT` reader - Receive Watchdog Timeout

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RWT_R = crate::BitReader;
///Field `RWT` writer - Receive Watchdog Timeout
pub type RWT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transmit Jabber Timeout
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - No Carrier
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Loss of Carrier
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Excessive Deferral
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Late Collision
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Excessive Collisions
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Receive Watchdog Timeout
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRXTXSR").finish()
    }
}
impl W {
    ///Bit 0 - Transmit Jabber Timeout
    #[inline(always)]
    pub fn tjt(&mut self) -> TJT_W<'_, MACRXTXSRrs> {
        TJT_W::new(self, 0)
    }
    ///Bit 1 - No Carrier
    #[inline(always)]
    pub fn ncarr(&mut self) -> NCARR_W<'_, MACRXTXSRrs> {
        NCARR_W::new(self, 1)
    }
    ///Bit 2 - Loss of Carrier
    #[inline(always)]
    pub fn lcarr(&mut self) -> LCARR_W<'_, MACRXTXSRrs> {
        LCARR_W::new(self, 2)
    }
    ///Bit 3 - Excessive Deferral
    #[inline(always)]
    pub fn exdef(&mut self) -> EXDEF_W<'_, MACRXTXSRrs> {
        EXDEF_W::new(self, 3)
    }
    ///Bit 4 - Late Collision
    #[inline(always)]
    pub fn lcol(&mut self) -> LCOL_W<'_, MACRXTXSRrs> {
        LCOL_W::new(self, 4)
    }
    ///Bit 5 - Excessive Collisions
    #[inline(always)]
    pub fn excol(&mut self) -> EXCOL_W<'_, MACRXTXSRrs> {
        EXCOL_W::new(self, 5)
    }
    ///Bit 8 - Receive Watchdog Timeout
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<'_, MACRXTXSRrs> {
        RWT_W::new(self, 8)
    }
}
/**Rx Tx status register

You can [`read`](crate::Reg::read) this register and get [`macrxtxsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrxtxsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACRXTXSR)*/
pub struct MACRXTXSRrs;
impl crate::RegisterSpec for MACRXTXSRrs {
    type Ux = u32;
}
///`read()` method returns [`macrxtxsr::R`](R) reader structure
impl crate::Readable for MACRXTXSRrs {}
///`write(|w| ..)` method takes [`macrxtxsr::W`](W) writer structure
impl crate::Writable for MACRXTXSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRXTXSR to value 0
impl crate::Resettable for MACRXTXSRrs {}
