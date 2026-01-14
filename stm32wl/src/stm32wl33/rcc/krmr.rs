///Register `KRMR` reader
pub type R = crate::R<KRMRrs>;
///Register `KRMR` writer
pub type W = crate::W<KRMRrs>;
///Field `KRM_EN` reader - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled.
pub type KRM_EN_R = crate::BitReader;
///Field `KRM_EN` writer - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled.
pub type KRM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KRM` reader - KRM\[4:0\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \[4-8\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL).
pub type KRM_R = crate::FieldReader;
///Field `KRM` writer - KRM\[4:0\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \[4-8\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL).
pub type KRM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled.
    #[inline(always)]
    pub fn krm_en(&self) -> KRM_EN_R {
        KRM_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:5 - KRM\[4:0\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \[4-8\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL).
    #[inline(always)]
    pub fn krm(&self) -> KRM_R {
        KRM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KRMR")
            .field("krm_en", &self.krm_en())
            .field("krm", &self.krm())
            .finish()
    }
}
impl W {
    ///Bit 0 - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled.
    #[inline(always)]
    pub fn krm_en(&mut self) -> KRM_EN_W<'_, KRMRrs> {
        KRM_EN_W::new(self, 0)
    }
    ///Bits 1:5 - KRM\[4:0\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \[4-8\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL).
    #[inline(always)]
    pub fn krm(&mut self) -> KRM_W<'_, KRMRrs> {
        KRM_W::new(self, 1)
    }
}
/**KRMR register

You can [`read`](crate::Reg::read) this register and get [`krmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:KRMR)*/
pub struct KRMRrs;
impl crate::RegisterSpec for KRMRrs {
    type Ux = u32;
}
///`read()` method returns [`krmr::R`](R) reader structure
impl crate::Readable for KRMRrs {}
///`write(|w| ..)` method takes [`krmr::W`](W) writer structure
impl crate::Writable for KRMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KRMR to value 0
impl crate::Resettable for KRMRrs {}
