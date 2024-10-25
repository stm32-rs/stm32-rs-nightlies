///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CECEN` reader - CECEN
pub type CECEN_R = crate::BitReader;
///Field `CECEN` writer - CECEN
pub type CECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSOM` reader - TXSOM
pub type TXSOM_R = crate::BitReader;
///Field `TXSOM` writer - TXSOM
pub type TXSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEOM` reader - TXEOM
pub type TXEOM_R = crate::BitReader;
///Field `TXEOM` writer - TXEOM
pub type TXEOM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CECEN
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXSOM
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXEOM
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("cecen", &self.cecen())
            .field("txsom", &self.txsom())
            .field("txeom", &self.txeom())
            .finish()
    }
}
impl W {
    ///Bit 0 - CECEN
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<CRrs> {
        CECEN_W::new(self, 0)
    }
    ///Bit 1 - TXSOM
    #[inline(always)]
    #[must_use]
    pub fn txsom(&mut self) -> TXSOM_W<CRrs> {
        TXSOM_W::new(self, 1)
    }
    ///Bit 2 - TXEOM
    #[inline(always)]
    #[must_use]
    pub fn txeom(&mut self) -> TXEOM_W<CRrs> {
        TXEOM_W::new(self, 2)
    }
}
/**CEC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDMI_CEC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
