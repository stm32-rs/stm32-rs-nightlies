#[doc = "Register `DSI_CMCR` reader"]
pub type R = crate::R<DSI_CMCRrs>;
#[doc = "Register `DSI_CMCR` writer"]
pub type W = crate::W<DSI_CMCRrs>;
#[doc = "Field `TEARE` reader - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:"]
pub type TEARE_R = crate::BitReader;
#[doc = "Field `TEARE` writer - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:"]
pub type TEARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARE` reader - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:"]
pub type ARE_R = crate::BitReader;
#[doc = "Field `ARE` writer - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:"]
pub type ARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW0TX` reader - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:"]
pub type GSW0TX_R = crate::BitReader;
#[doc = "Field `GSW0TX` writer - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:"]
pub type GSW0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW1TX` reader - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:"]
pub type GSW1TX_R = crate::BitReader;
#[doc = "Field `GSW1TX` writer - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:"]
pub type GSW1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW2TX` reader - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:"]
pub type GSW2TX_R = crate::BitReader;
#[doc = "Field `GSW2TX` writer - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:"]
pub type GSW2TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR0TX` reader - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:"]
pub type GSR0TX_R = crate::BitReader;
#[doc = "Field `GSR0TX` writer - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:"]
pub type GSR0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR1TX` reader - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:"]
pub type GSR1TX_R = crate::BitReader;
#[doc = "Field `GSR1TX` writer - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:"]
pub type GSR1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR2TX` reader - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:"]
pub type GSR2TX_R = crate::BitReader;
#[doc = "Field `GSR2TX` writer - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:"]
pub type GSR2TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLWTX` reader - Generic long write transmission This bit configures the generic long write packet command transmission type :"]
pub type GLWTX_R = crate::BitReader;
#[doc = "Field `GLWTX` writer - Generic long write transmission This bit configures the generic long write packet command transmission type :"]
pub type GLWTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSW0TX` reader - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:"]
pub type DSW0TX_R = crate::BitReader;
#[doc = "Field `DSW0TX` writer - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:"]
pub type DSW0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSW1TX` reader - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:"]
pub type DSW1TX_R = crate::BitReader;
#[doc = "Field `DSW1TX` writer - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:"]
pub type DSW1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR0TX` reader - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:"]
pub type DSR0TX_R = crate::BitReader;
#[doc = "Field `DSR0TX` writer - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:"]
pub type DSR0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLWTX` reader - DCS long write transmission This bit configures the DCS long write packet command transmission type:"]
pub type DLWTX_R = crate::BitReader;
#[doc = "Field `DLWTX` writer - DCS long write transmission This bit configures the DCS long write packet command transmission type:"]
pub type DLWTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRDPS` reader - Maximum read packet size This bit configures the maximum read packet size command transmission type:"]
pub type MRDPS_R = crate::BitReader;
#[doc = "Field `MRDPS` writer - Maximum read packet size This bit configures the maximum read packet size command transmission type:"]
pub type MRDPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:"]
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:"]
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:"]
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:"]
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:"]
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:"]
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:"]
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:"]
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generic long write transmission This bit configures the generic long write packet command transmission type :"]
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:"]
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:"]
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:"]
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCS long write transmission This bit configures the DCS long write packet command transmission type:"]
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Maximum read packet size This bit configures the maximum read packet size command transmission type:"]
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:"]
    #[inline(always)]
    #[must_use]
    pub fn teare(&mut self) -> TEARE_W<DSI_CMCRrs> {
        TEARE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:"]
    #[inline(always)]
    #[must_use]
    pub fn are(&mut self) -> ARE_W<DSI_CMCRrs> {
        ARE_W::new(self, 1)
    }
    #[doc = "Bit 8 - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn gsw0tx(&mut self) -> GSW0TX_W<DSI_CMCRrs> {
        GSW0TX_W::new(self, 8)
    }
    #[doc = "Bit 9 - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn gsw1tx(&mut self) -> GSW1TX_W<DSI_CMCRrs> {
        GSW1TX_W::new(self, 9)
    }
    #[doc = "Bit 10 - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn gsw2tx(&mut self) -> GSW2TX_W<DSI_CMCRrs> {
        GSW2TX_W::new(self, 10)
    }
    #[doc = "Bit 11 - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn gsr0tx(&mut self) -> GSR0TX_W<DSI_CMCRrs> {
        GSR0TX_W::new(self, 11)
    }
    #[doc = "Bit 12 - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn gsr1tx(&mut self) -> GSR1TX_W<DSI_CMCRrs> {
        GSR1TX_W::new(self, 12)
    }
    #[doc = "Bit 13 - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn gsr2tx(&mut self) -> GSR2TX_W<DSI_CMCRrs> {
        GSR2TX_W::new(self, 13)
    }
    #[doc = "Bit 14 - Generic long write transmission This bit configures the generic long write packet command transmission type :"]
    #[inline(always)]
    #[must_use]
    pub fn glwtx(&mut self) -> GLWTX_W<DSI_CMCRrs> {
        GLWTX_W::new(self, 14)
    }
    #[doc = "Bit 16 - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn dsw0tx(&mut self) -> DSW0TX_W<DSI_CMCRrs> {
        DSW0TX_W::new(self, 16)
    }
    #[doc = "Bit 17 - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn dsw1tx(&mut self) -> DSW1TX_W<DSI_CMCRrs> {
        DSW1TX_W::new(self, 17)
    }
    #[doc = "Bit 18 - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn dsr0tx(&mut self) -> DSR0TX_W<DSI_CMCRrs> {
        DSR0TX_W::new(self, 18)
    }
    #[doc = "Bit 19 - DCS long write transmission This bit configures the DCS long write packet command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn dlwtx(&mut self) -> DLWTX_W<DSI_CMCRrs> {
        DLWTX_W::new(self, 19)
    }
    #[doc = "Bit 24 - Maximum read packet size This bit configures the maximum read packet size command transmission type:"]
    #[inline(always)]
    #[must_use]
    pub fn mrdps(&mut self) -> MRDPS_W<DSI_CMCRrs> {
        MRDPS_W::new(self, 24)
    }
}
#[doc = "DSI Host command mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CMCRrs;
impl crate::RegisterSpec for DSI_CMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_cmcr::R`](R) reader structure"]
impl crate::Readable for DSI_CMCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_cmcr::W`](W) writer structure"]
impl crate::Writable for DSI_CMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_CMCR to value 0"]
impl crate::Resettable for DSI_CMCRrs {
    const RESET_VALUE: u32 = 0;
}
