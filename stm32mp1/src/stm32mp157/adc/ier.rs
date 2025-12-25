///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `ADRDYIE` reader - ADRDYIE
pub type ADRDYIE_R = crate::BitReader;
///Field `ADRDYIE` writer - ADRDYIE
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSMPIE` reader - EOSMPIE
pub type EOSMPIE_R = crate::BitReader;
///Field `EOSMPIE` writer - EOSMPIE
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCIE` reader - EOCIE
pub type EOCIE_R = crate::BitReader;
///Field `EOCIE` writer - EOCIE
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOSIE` reader - EOSIE
pub type EOSIE_R = crate::BitReader;
///Field `EOSIE` writer - EOSIE
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - OVRIE
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - OVRIE
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOCIE` reader - JEOCIE
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - JEOCIE
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOSIE` reader - JEOSIE
pub type JEOSIE_R = crate::BitReader;
///Field `JEOSIE` writer - JEOSIE
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD1IE` reader - AWD1IE
pub type AWD1IE_R = crate::BitReader;
///Field `AWD1IE` writer - AWD1IE
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD2IE` reader - AWD2IE
pub type AWD2IE_R = crate::BitReader;
///Field `AWD2IE` writer - AWD2IE
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD3IE` reader - AWD3IE
pub type AWD3IE_R = crate::BitReader;
///Field `AWD3IE` writer - AWD3IE
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JQOVFIE` reader - JQOVFIE
pub type JQOVFIE_R = crate::BitReader;
///Field `JQOVFIE` writer - JQOVFIE
pub type JQOVFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JEOCIE
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - JEOSIE
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AWD1IE
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD2IE
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD3IE
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - JQOVFIE
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("adrdyie", &self.adrdyie())
            .field("eosmpie", &self.eosmpie())
            .field("eocie", &self.eocie())
            .field("eosie", &self.eosie())
            .field("ovrie", &self.ovrie())
            .field("jeocie", &self.jeocie())
            .field("jeosie", &self.jeosie())
            .field("awd1ie", &self.awd1ie())
            .field("awd2ie", &self.awd2ie())
            .field("awd3ie", &self.awd3ie())
            .field("jqovfie", &self.jqovfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<'_, IERrs> {
        ADRDYIE_W::new(self, 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<'_, IERrs> {
        EOSMPIE_W::new(self, 1)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<'_, IERrs> {
        EOCIE_W::new(self, 2)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W<'_, IERrs> {
        EOSIE_W::new(self, 3)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<'_, IERrs> {
        OVRIE_W::new(self, 4)
    }
    ///Bit 5 - JEOCIE
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<'_, IERrs> {
        JEOCIE_W::new(self, 5)
    }
    ///Bit 6 - JEOSIE
    #[inline(always)]
    pub fn jeosie(&mut self) -> JEOSIE_W<'_, IERrs> {
        JEOSIE_W::new(self, 6)
    }
    ///Bit 7 - AWD1IE
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWD1IE_W<'_, IERrs> {
        AWD1IE_W::new(self, 7)
    }
    ///Bit 8 - AWD2IE
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWD2IE_W<'_, IERrs> {
        AWD2IE_W::new(self, 8)
    }
    ///Bit 9 - AWD3IE
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWD3IE_W<'_, IERrs> {
        AWD3IE_W::new(self, 9)
    }
    ///Bit 10 - JQOVFIE
    #[inline(always)]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<'_, IERrs> {
        JQOVFIE_W::new(self, 10)
    }
}
/**ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
