#[doc = "Register `CMCR` reader"]
pub type R = crate::R<CMCRrs>;
#[doc = "Register `CMCR` writer"]
pub type W = crate::W<CMCRrs>;
#[doc = "Field `TEARE` reader - TEARE"]
pub type TEARE_R = crate::BitReader;
#[doc = "Field `TEARE` writer - TEARE"]
pub type TEARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARE` reader - ARE"]
pub type ARE_R = crate::BitReader;
#[doc = "Field `ARE` writer - ARE"]
pub type ARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW0TX` reader - GSW0TX"]
pub type GSW0TX_R = crate::BitReader;
#[doc = "Field `GSW0TX` writer - GSW0TX"]
pub type GSW0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW1TX` reader - GSW1TX"]
pub type GSW1TX_R = crate::BitReader;
#[doc = "Field `GSW1TX` writer - GSW1TX"]
pub type GSW1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW2TX` reader - GSW2TX"]
pub type GSW2TX_R = crate::BitReader;
#[doc = "Field `GSW2TX` writer - GSW2TX"]
pub type GSW2TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR0TX` reader - GSR0TX"]
pub type GSR0TX_R = crate::BitReader;
#[doc = "Field `GSR0TX` writer - GSR0TX"]
pub type GSR0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR1TX` reader - GSR1TX"]
pub type GSR1TX_R = crate::BitReader;
#[doc = "Field `GSR1TX` writer - GSR1TX"]
pub type GSR1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR2TX` reader - GSR2TX"]
pub type GSR2TX_R = crate::BitReader;
#[doc = "Field `GSR2TX` writer - GSR2TX"]
pub type GSR2TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLWTX` reader - GLWTX"]
pub type GLWTX_R = crate::BitReader;
#[doc = "Field `GLWTX` writer - GLWTX"]
pub type GLWTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSW0TX` reader - DSW0TX"]
pub type DSW0TX_R = crate::BitReader;
#[doc = "Field `DSW0TX` writer - DSW0TX"]
pub type DSW0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSW1TX` reader - DSW1TX"]
pub type DSW1TX_R = crate::BitReader;
#[doc = "Field `DSW1TX` writer - DSW1TX"]
pub type DSW1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR0TX` reader - DSR0TX"]
pub type DSR0TX_R = crate::BitReader;
#[doc = "Field `DSR0TX` writer - DSR0TX"]
pub type DSR0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLWTX` reader - DLWTX"]
pub type DLWTX_R = crate::BitReader;
#[doc = "Field `DLWTX` writer - DLWTX"]
pub type DLWTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRDPS` reader - MRDPS"]
pub type MRDPS_R = crate::BitReader;
#[doc = "Field `MRDPS` writer - MRDPS"]
pub type MRDPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    #[must_use]
    pub fn teare(&mut self) -> TEARE_W<CMCRrs> {
        TEARE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    #[must_use]
    pub fn are(&mut self) -> ARE_W<CMCRrs> {
        ARE_W::new(self, 1)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsw0tx(&mut self) -> GSW0TX_W<CMCRrs> {
        GSW0TX_W::new(self, 8)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsw1tx(&mut self) -> GSW1TX_W<CMCRrs> {
        GSW1TX_W::new(self, 9)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsw2tx(&mut self) -> GSW2TX_W<CMCRrs> {
        GSW2TX_W::new(self, 10)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsr0tx(&mut self) -> GSR0TX_W<CMCRrs> {
        GSR0TX_W::new(self, 11)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsr1tx(&mut self) -> GSR1TX_W<CMCRrs> {
        GSR1TX_W::new(self, 12)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsr2tx(&mut self) -> GSR2TX_W<CMCRrs> {
        GSR2TX_W::new(self, 13)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    #[must_use]
    pub fn glwtx(&mut self) -> GLWTX_W<CMCRrs> {
        GLWTX_W::new(self, 14)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    #[must_use]
    pub fn dsw0tx(&mut self) -> DSW0TX_W<CMCRrs> {
        DSW0TX_W::new(self, 16)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    #[must_use]
    pub fn dsw1tx(&mut self) -> DSW1TX_W<CMCRrs> {
        DSW1TX_W::new(self, 17)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    #[must_use]
    pub fn dsr0tx(&mut self) -> DSR0TX_W<CMCRrs> {
        DSR0TX_W::new(self, 18)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    #[must_use]
    pub fn dlwtx(&mut self) -> DLWTX_W<CMCRrs> {
        DLWTX_W::new(self, 19)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    #[must_use]
    pub fn mrdps(&mut self) -> MRDPS_W<CMCRrs> {
        MRDPS_W::new(self, 24)
    }
}
#[doc = "DSI Host command mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMCRrs;
impl crate::RegisterSpec for CMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmcr::R`](R) reader structure"]
impl crate::Readable for CMCRrs {}
#[doc = "`write(|w| ..)` method takes [`cmcr::W`](W) writer structure"]
impl crate::Writable for CMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMCR to value 0"]
impl crate::Resettable for CMCRrs {
    const RESET_VALUE: u32 = 0;
}
