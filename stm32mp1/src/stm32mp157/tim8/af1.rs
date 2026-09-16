///Register `AF1` reader
pub type R = crate::R<AF1rs>;
///Register `AF1` writer
pub type W = crate::W<AF1rs>;
///Field `BKINE` reader - BKINE
pub type BKINE_R = crate::BitReader;
///Field `BKINE` writer - BKINE
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKDF1BK2E` reader - BKDF1BK2E
pub type BKDF1BK2E_R = crate::BitReader;
///Field `BKDF1BK2E` writer - BKDF1BK2E
pub type BKDF1BK2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKINP` reader - BKINP
pub type BKINP_R = crate::BitReader;
///Field `BKINP` writer - BKINP
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETRSEL` reader - ETRSEL
pub type ETRSEL_R = crate::FieldReader;
///Field `ETRSEL` writer - ETRSEL
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - BKINE
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - BKDF1BK2E
    #[inline(always)]
    pub fn bkdf1bk2e(&self) -> BKDF1BK2E_R {
        BKDF1BK2E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BKINP
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 14:17 - ETRSEL
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF1")
            .field("bkine", &self.bkine())
            .field("bkdf1bk2e", &self.bkdf1bk2e())
            .field("bkinp", &self.bkinp())
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - BKINE
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W<'_, AF1rs> {
        BKINE_W::new(self, 0)
    }
    ///Bit 8 - BKDF1BK2E
    #[inline(always)]
    pub fn bkdf1bk2e(&mut self) -> BKDF1BK2E_W<'_, AF1rs> {
        BKDF1BK2E_W::new(self, 8)
    }
    ///Bit 9 - BKINP
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W<'_, AF1rs> {
        BKINP_W::new(self, 9)
    }
    ///Bits 14:17 - ETRSEL
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<'_, AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM8 Alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM8:AF1)*/
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
///`read()` method returns [`af1::R`](R) reader structure
impl crate::Readable for AF1rs {}
///`write(|w| ..)` method takes [`af1::W`](W) writer structure
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF1 to value 0x01
impl crate::Resettable for AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
