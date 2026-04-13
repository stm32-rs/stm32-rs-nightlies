///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LSION` reader - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDY` reader - Internal Low Speed oscillator Ready Set and reset by hardware to indicate when the Low Speed Internal RC oscillator is stable. Reset source only for this field: PORESETn
pub type LSIRDY_R = crate::BitReader;
///Field `LSEON` reader - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDY` reader - External Low Speed Clock ready flag. Set by hardware to indicate that LSE oscillator is stable.
pub type LSERDY_R = crate::BitReader;
///Field `LSEBYP` reader - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKDET_NSTOP` reader - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0.
pub type LOCKDET_NSTOP_R = crate::FieldReader;
///Field `LOCKDET_NSTOP` writer - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0.
pub type LOCKDET_NSTOP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HSIRDY` reader - Internal High Speed clock ready flag. Set by hardware to indicate that internal RC 64MHz oscillator is stable. This bit is activated only if the RC is enabled by HSION (it is not activated if the RC is enabled by an IP request).
pub type HSIRDY_R = crate::BitReader;
///Field `HSEPLLBUFON` reader - External High Speed Clock Buffer for PLL RF2G4 enable. Set and reset by software.
pub type HSEPLLBUFON_R = crate::BitReader;
///Field `HSEPLLBUFON` writer - External High Speed Clock Buffer for PLL RF2G4 enable. Set and reset by software.
pub type HSEPLLBUFON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPLLON` reader - Internal High Speed Clock PLL enable
pub type HSIPLLON_R = crate::BitReader;
///Field `HSIPLLON` writer - Internal High Speed Clock PLL enable
pub type HSIPLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPLLRDY` reader - Internal High Speed Clock PLL ready flag.
pub type HSIPLLRDY_R = crate::BitReader;
///Field `HSEON` reader - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off.
pub type HSEON_R = crate::BitReader;
///Field `HSEON` writer - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off.
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDY` reader - External High Speed Clock ready flag. Set by hardware to indicate that HSE oscillator is stable.
pub type HSERDY_R = crate::BitReader;
impl R {
    ///Bit 2 - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Internal Low Speed oscillator Ready Set and reset by hardware to indicate when the Low Speed Internal RC oscillator is stable. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - External Low Speed Clock ready flag. Set by hardware to indicate that LSE oscillator is stable.
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:9 - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0.
    #[inline(always)]
    pub fn lockdet_nstop(&self) -> LOCKDET_NSTOP_R {
        LOCKDET_NSTOP_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bit 10 - Internal High Speed clock ready flag. Set by hardware to indicate that internal RC 64MHz oscillator is stable. This bit is activated only if the RC is enabled by HSION (it is not activated if the RC is enabled by an IP request).
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - External High Speed Clock Buffer for PLL RF2G4 enable. Set and reset by software.
    #[inline(always)]
    pub fn hsepllbufon(&self) -> HSEPLLBUFON_R {
        HSEPLLBUFON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Internal High Speed Clock PLL enable
    #[inline(always)]
    pub fn hsipllon(&self) -> HSIPLLON_R {
        HSIPLLON_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Internal High Speed Clock PLL ready flag.
    #[inline(always)]
    pub fn hsipllrdy(&self) -> HSIPLLRDY_R {
        HSIPLLRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off.
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - External High Speed Clock ready flag. Set by hardware to indicate that HSE oscillator is stable.
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("lockdet_nstop", &self.lockdet_nstop())
            .field("hsirdy", &self.hsirdy())
            .field("hsepllbufon", &self.hsepllbufon())
            .field("hsipllon", &self.hsipllon())
            .field("hsipllrdy", &self.hsipllrdy())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .finish()
    }
}
impl W {
    ///Bit 2 - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CRrs> {
        LSION_W::new(self, 2)
    }
    ///Bit 4 - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, CRrs> {
        LSEON_W::new(self, 4)
    }
    ///Bit 6 - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, CRrs> {
        LSEBYP_W::new(self, 6)
    }
    ///Bits 7:9 - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0.
    #[inline(always)]
    pub fn lockdet_nstop(&mut self) -> LOCKDET_NSTOP_W<'_, CRrs> {
        LOCKDET_NSTOP_W::new(self, 7)
    }
    ///Bit 12 - External High Speed Clock Buffer for PLL RF2G4 enable. Set and reset by software.
    #[inline(always)]
    pub fn hsepllbufon(&mut self) -> HSEPLLBUFON_W<'_, CRrs> {
        HSEPLLBUFON_W::new(self, 12)
    }
    ///Bit 13 - Internal High Speed Clock PLL enable
    #[inline(always)]
    pub fn hsipllon(&mut self) -> HSIPLLON_W<'_, CRrs> {
        HSIPLLON_W::new(self, 13)
    }
    ///Bit 16 - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off.
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
}
/**CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RCC:CR)*/
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
///`reset()` method sets CR to value 0x1400
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x1400;
}
