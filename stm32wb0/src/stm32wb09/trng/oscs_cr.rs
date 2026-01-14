///Register `OSCS_CR` reader
pub type R = crate::R<OSCS_CRrs>;
///Register `OSCS_CR` writer
pub type W = crate::W<OSCS_CRrs>;
///Field `PWRD1` reader - Power down of individual oscillators in triple-oscillator block number 1
pub type PWRD1_R = crate::FieldReader;
///Field `PWRD1` writer - Power down of individual oscillators in triple-oscillator block number 1
pub type PWRD1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PWRD2` reader - Power down of individual oscillators in triple-oscillator block number 2
pub type PWRD2_R = crate::FieldReader;
///Field `PWRD2` writer - Power down of individual oscillators in triple-oscillator block number 2
pub type PWRD2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PWRD3` reader - Power down of individual oscillators in triple-oscillator block number 3
pub type PWRD3_R = crate::FieldReader;
///Field `PWRD3` writer - Power down of individual oscillators in triple-oscillator block number 3
pub type PWRD3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SYNC_OSCS` reader - When set, selection of resynchronized output of oscillators.
pub type SYNC_OSCS_R = crate::BitReader;
///Field `SYNC_OSCS` writer - When set, selection of resynchronized output of oscillators.
pub type SYNC_OSCS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 1:3 - Power down of individual oscillators in triple-oscillator block number 1
    #[inline(always)]
    pub fn pwrd1(&self) -> PWRD1_R {
        PWRD1_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:6 - Power down of individual oscillators in triple-oscillator block number 2
    #[inline(always)]
    pub fn pwrd2(&self) -> PWRD2_R {
        PWRD2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9 - Power down of individual oscillators in triple-oscillator block number 3
    #[inline(always)]
    pub fn pwrd3(&self) -> PWRD3_R {
        PWRD3_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bit 31 - When set, selection of resynchronized output of oscillators.
    #[inline(always)]
    pub fn sync_oscs(&self) -> SYNC_OSCS_R {
        SYNC_OSCS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSCS_CR")
            .field("pwrd1", &self.pwrd1())
            .field("pwrd2", &self.pwrd2())
            .field("pwrd3", &self.pwrd3())
            .field("sync_oscs", &self.sync_oscs())
            .finish()
    }
}
impl W {
    ///Bits 1:3 - Power down of individual oscillators in triple-oscillator block number 1
    #[inline(always)]
    pub fn pwrd1(&mut self) -> PWRD1_W<'_, OSCS_CRrs> {
        PWRD1_W::new(self, 1)
    }
    ///Bits 4:6 - Power down of individual oscillators in triple-oscillator block number 2
    #[inline(always)]
    pub fn pwrd2(&mut self) -> PWRD2_W<'_, OSCS_CRrs> {
        PWRD2_W::new(self, 4)
    }
    ///Bits 7:9 - Power down of individual oscillators in triple-oscillator block number 3
    #[inline(always)]
    pub fn pwrd3(&mut self) -> PWRD3_W<'_, OSCS_CRrs> {
        PWRD3_W::new(self, 7)
    }
    ///Bit 31 - When set, selection of resynchronized output of oscillators.
    #[inline(always)]
    pub fn sync_oscs(&mut self) -> SYNC_OSCS_W<'_, OSCS_CRrs> {
        SYNC_OSCS_W::new(self, 31)
    }
}
/**TRNG_OSCS_CR register

You can [`read`](crate::Reg::read) this register and get [`oscs_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscs_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:OSCS_CR)*/
pub struct OSCS_CRrs;
impl crate::RegisterSpec for OSCS_CRrs {
    type Ux = u32;
}
///`read()` method returns [`oscs_cr::R`](R) reader structure
impl crate::Readable for OSCS_CRrs {}
///`write(|w| ..)` method takes [`oscs_cr::W`](W) writer structure
impl crate::Writable for OSCS_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSCS_CR to value 0x8000_0000
impl crate::Resettable for OSCS_CRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
