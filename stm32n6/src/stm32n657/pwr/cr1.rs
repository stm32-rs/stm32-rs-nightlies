///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `SDEN` reader - SMPS step-down converter enable
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - SMPS step-down converter enable
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE_PDN` reader - Enables the pull down on output voltage during power-down mode
pub type MODE_PDN_R = crate::BitReader;
///Field `MODE_PDN` writer - Enables the pull down on output voltage during power-down mode
pub type MODE_PDN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPDS08V` reader - SMPS low-power mode enable (SVOS high only)
pub type LPDS08V_R = crate::BitReader;
///Field `LPDS08V` writer - SMPS low-power mode enable (SVOS high only)
pub type LPDS08V_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POPL` reader - pwr_on pulse low configuration.
pub type POPL_R = crate::FieldReader;
///Field `POPL` writer - pwr_on pulse low configuration.
pub type POPL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 2 - SMPS step-down converter enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Enables the pull down on output voltage during power-down mode
    #[inline(always)]
    pub fn mode_pdn(&self) -> MODE_PDN_R {
        MODE_PDN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SMPS low-power mode enable (SVOS high only)
    #[inline(always)]
    pub fn lpds08v(&self) -> LPDS08V_R {
        LPDS08V_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 16:20 - pwr_on pulse low configuration.
    #[inline(always)]
    pub fn popl(&self) -> POPL_R {
        POPL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("sden", &self.sden())
            .field("mode_pdn", &self.mode_pdn())
            .field("lpds08v", &self.lpds08v())
            .field("popl", &self.popl())
            .finish()
    }
}
impl W {
    ///Bit 2 - SMPS step-down converter enable
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<'_, CR1rs> {
        SDEN_W::new(self, 2)
    }
    ///Bit 4 - Enables the pull down on output voltage during power-down mode
    #[inline(always)]
    pub fn mode_pdn(&mut self) -> MODE_PDN_W<'_, CR1rs> {
        MODE_PDN_W::new(self, 4)
    }
    ///Bit 5 - SMPS low-power mode enable (SVOS high only)
    #[inline(always)]
    pub fn lpds08v(&mut self) -> LPDS08V_W<'_, CR1rs> {
        LPDS08V_W::new(self, 5)
    }
    ///Bits 16:20 - pwr_on pulse low configuration.
    #[inline(always)]
    pub fn popl(&mut self) -> POPL_W<'_, CR1rs> {
        POPL_W::new(self, 16)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0x24
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x24;
}
