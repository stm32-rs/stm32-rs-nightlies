///Register `_AF1` reader
pub type R = crate::R<_AF1rs>;
///Register `_AF1` writer
pub type W = crate::W<_AF1rs>;
///Field `BKINE` reader - BKINE
pub type BKINE_R = crate::BitReader;
///Field `BKINE` writer - BKINE
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKDF1BK0E` reader - BKDF1BK0E
pub type BKDF1BK0E_R = crate::BitReader;
///Field `BKDF1BK0E` writer - BKDF1BK0E
pub type BKDF1BK0E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKINP` reader - BKINP
pub type BKINP_R = crate::BitReader;
///Field `BKINP` writer - BKINP
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BKINE
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - BKDF1BK0E
    #[inline(always)]
    pub fn bkdf1bk0e(&self) -> BKDF1BK0E_R {
        BKDF1BK0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BKINP
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_AF1")
            .field("bkine", &self.bkine())
            .field("bkdf1bk0e", &self.bkdf1bk0e())
            .field("bkinp", &self.bkinp())
            .finish()
    }
}
impl W {
    ///Bit 0 - BKINE
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W<'_, _AF1rs> {
        BKINE_W::new(self, 0)
    }
    ///Bit 8 - BKDF1BK0E
    #[inline(always)]
    pub fn bkdf1bk0e(&mut self) -> BKDF1BK0E_W<'_, _AF1rs> {
        BKDF1BK0E_W::new(self, 8)
    }
    ///Bit 9 - BKINP
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W<'_, _AF1rs> {
        BKINP_W::new(self, 9)
    }
}
/**TIM15 alternate register 1

You can [`read`](crate::Reg::read) this register and get [`_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM15:_AF1)*/
pub struct _AF1rs;
impl crate::RegisterSpec for _AF1rs {
    type Ux = u32;
}
///`read()` method returns [`_af1::R`](R) reader structure
impl crate::Readable for _AF1rs {}
///`write(|w| ..)` method takes [`_af1::W`](W) writer structure
impl crate::Writable for _AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _AF1 to value 0x01
impl crate::Resettable for _AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
