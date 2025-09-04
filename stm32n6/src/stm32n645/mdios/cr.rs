///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - peripheral enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - peripheral enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRIE` reader - register write interrupt enable
pub type WRIE_R = crate::BitReader;
///Field `WRIE` writer - register write interrupt enable
pub type WRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDIE` reader - register read interrupt enable
pub type RDIE_R = crate::BitReader;
///Field `RDIE` writer - register read interrupt enable
pub type RDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIE` reader - error interrupt enable
pub type EIE_R = crate::BitReader;
///Field `EIE` writer - error interrupt enable
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPC` reader - disable preamble check
pub type DPC_R = crate::BitReader;
///Field `DPC` writer - disable preamble check
pub type DPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORT_ADDRESS` reader - slave address
pub type PORT_ADDRESS_R = crate::FieldReader;
///Field `PORT_ADDRESS` writer - slave address
pub type PORT_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - peripheral enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - register write interrupt enable
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - register read interrupt enable
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - disable preamble check
    #[inline(always)]
    pub fn dpc(&self) -> DPC_R {
        DPC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - slave address
    #[inline(always)]
    pub fn port_address(&self) -> PORT_ADDRESS_R {
        PORT_ADDRESS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("wrie", &self.wrie())
            .field("rdie", &self.rdie())
            .field("eie", &self.eie())
            .field("dpc", &self.dpc())
            .field("port_address", &self.port_address())
            .finish()
    }
}
impl W {
    ///Bit 0 - peripheral enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - register write interrupt enable
    #[inline(always)]
    pub fn wrie(&mut self) -> WRIE_W<CRrs> {
        WRIE_W::new(self, 1)
    }
    ///Bit 2 - register read interrupt enable
    #[inline(always)]
    pub fn rdie(&mut self) -> RDIE_W<CRrs> {
        RDIE_W::new(self, 2)
    }
    ///Bit 3 - error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<CRrs> {
        EIE_W::new(self, 3)
    }
    ///Bit 7 - disable preamble check
    #[inline(always)]
    pub fn dpc(&mut self) -> DPC_W<CRrs> {
        DPC_W::new(self, 7)
    }
    ///Bits 8:12 - slave address
    #[inline(always)]
    pub fn port_address(&mut self) -> PORT_ADDRESS_W<CRrs> {
        PORT_ADDRESS_W::new(self, 8)
    }
}
/**MDIOS configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDIOS:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
