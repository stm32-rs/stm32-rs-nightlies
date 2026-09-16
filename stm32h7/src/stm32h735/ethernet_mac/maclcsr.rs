///Register `MACLCSR` reader
pub type R = crate::R<MACLCSRrs>;
///Register `MACLCSR` writer
pub type W = crate::W<MACLCSRrs>;
///Field `TLPIEN` reader - Transmit LPI Entry
pub type TLPIEN_R = crate::BitReader;
///Field `TLPIEX` reader - Transmit LPI Exit
pub type TLPIEX_R = crate::BitReader;
///Field `RLPIEN` reader - Receive LPI Entry
pub type RLPIEN_R = crate::BitReader;
///Field `RLPIEX` reader - Receive LPI Exit
pub type RLPIEX_R = crate::BitReader;
///Field `TLPIST` reader - Transmit LPI State
pub type TLPIST_R = crate::BitReader;
///Field `RLPIST` reader - Receive LPI State
pub type RLPIST_R = crate::BitReader;
///Field `LPIEN` reader - LPI Enable
pub type LPIEN_R = crate::BitReader;
///Field `LPIEN` writer - LPI Enable
pub type LPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - PHY Link Status
pub type PLS_R = crate::BitReader;
///Field `PLS` writer - PHY Link Status
pub type PLS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLSEN` reader - PHY Link Status Enable
pub type PLSEN_R = crate::BitReader;
///Field `PLSEN` writer - PHY Link Status Enable
pub type PLSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPITXA` reader - LPI Tx Automate
pub type LPITXA_R = crate::BitReader;
///Field `LPITXA` writer - LPI Tx Automate
pub type LPITXA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPITE` reader - LPI Timer Enable
pub type LPITE_R = crate::BitReader;
///Field `LPITE` writer - LPI Timer Enable
pub type LPITE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPITCSE` reader - LPITCSE
pub type LPITCSE_R = crate::BitReader;
///Field `LPITCSE` writer - LPITCSE
pub type LPITCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transmit LPI Entry
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit LPI Exit
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive LPI Entry
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receive LPI Exit
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Transmit LPI State
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Receive LPI State
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - LPI Enable
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PHY Link Status
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PHY Link Status Enable
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LPI Tx Automate
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - LPI Timer Enable
    #[inline(always)]
    pub fn lpite(&self) -> LPITE_R {
        LPITE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - LPITCSE
    #[inline(always)]
    pub fn lpitcse(&self) -> LPITCSE_R {
        LPITCSE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACLCSR")
            .field("tlpien", &self.tlpien())
            .field("tlpiex", &self.tlpiex())
            .field("rlpien", &self.rlpien())
            .field("rlpiex", &self.rlpiex())
            .field("tlpist", &self.tlpist())
            .field("rlpist", &self.rlpist())
            .field("lpien", &self.lpien())
            .field("pls", &self.pls())
            .field("plsen", &self.plsen())
            .field("lpitxa", &self.lpitxa())
            .field("lpite", &self.lpite())
            .field("lpitcse", &self.lpitcse())
            .finish()
    }
}
impl W {
    ///Bit 16 - LPI Enable
    #[inline(always)]
    pub fn lpien(&mut self) -> LPIEN_W<'_, MACLCSRrs> {
        LPIEN_W::new(self, 16)
    }
    ///Bit 17 - PHY Link Status
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, MACLCSRrs> {
        PLS_W::new(self, 17)
    }
    ///Bit 18 - PHY Link Status Enable
    #[inline(always)]
    pub fn plsen(&mut self) -> PLSEN_W<'_, MACLCSRrs> {
        PLSEN_W::new(self, 18)
    }
    ///Bit 19 - LPI Tx Automate
    #[inline(always)]
    pub fn lpitxa(&mut self) -> LPITXA_W<'_, MACLCSRrs> {
        LPITXA_W::new(self, 19)
    }
    ///Bit 20 - LPI Timer Enable
    #[inline(always)]
    pub fn lpite(&mut self) -> LPITE_W<'_, MACLCSRrs> {
        LPITE_W::new(self, 20)
    }
    ///Bit 21 - LPITCSE
    #[inline(always)]
    pub fn lpitcse(&mut self) -> LPITCSE_W<'_, MACLCSRrs> {
        LPITCSE_W::new(self, 21)
    }
}
/**LPI control status register

You can [`read`](crate::Reg::read) this register and get [`maclcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#Ethernet_MAC:MACLCSR)*/
pub struct MACLCSRrs;
impl crate::RegisterSpec for MACLCSRrs {
    type Ux = u32;
}
///`read()` method returns [`maclcsr::R`](R) reader structure
impl crate::Readable for MACLCSRrs {}
///`write(|w| ..)` method takes [`maclcsr::W`](W) writer structure
impl crate::Writable for MACLCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACLCSR to value 0
impl crate::Resettable for MACLCSRrs {}
