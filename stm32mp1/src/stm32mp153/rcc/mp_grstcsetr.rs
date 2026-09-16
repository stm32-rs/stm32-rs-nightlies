///Register `MP_GRSTCSETR` reader
pub type R = crate::R<MP_GRSTCSETRrs>;
///Register `MP_GRSTCSETR` writer
pub type W = crate::W<MP_GRSTCSETRrs>;
///Field `MPSYSRST` reader - MPSYSRST
pub type MPSYSRST_R = crate::BitReader;
///Field `MPSYSRST` writer - MPSYSRST
pub type MPSYSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCURST` reader - MCURST
pub type MCURST_R = crate::BitReader;
///Field `MCURST` writer - MCURST
pub type MCURST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPUP0RST` reader - MPUP0RST
pub type MPUP0RST_R = crate::BitReader;
///Field `MPUP0RST` writer - MPUP0RST
pub type MPUP0RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPUP1RST` reader - MPUP1RST
pub type MPUP1RST_R = crate::BitReader;
///Field `MPUP1RST` writer - MPUP1RST
pub type MPUP1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MPSYSRST
    #[inline(always)]
    pub fn mpsysrst(&self) -> MPSYSRST_R {
        MPSYSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCURST
    #[inline(always)]
    pub fn mcurst(&self) -> MCURST_R {
        MCURST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - MPUP0RST
    #[inline(always)]
    pub fn mpup0rst(&self) -> MPUP0RST_R {
        MPUP0RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MPUP1RST
    #[inline(always)]
    pub fn mpup1rst(&self) -> MPUP1RST_R {
        MPUP1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_GRSTCSETR")
            .field("mpsysrst", &self.mpsysrst())
            .field("mcurst", &self.mcurst())
            .field("mpup0rst", &self.mpup0rst())
            .field("mpup1rst", &self.mpup1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - MPSYSRST
    #[inline(always)]
    pub fn mpsysrst(&mut self) -> MPSYSRST_W<'_, MP_GRSTCSETRrs> {
        MPSYSRST_W::new(self, 0)
    }
    ///Bit 1 - MCURST
    #[inline(always)]
    pub fn mcurst(&mut self) -> MCURST_W<'_, MP_GRSTCSETRrs> {
        MCURST_W::new(self, 1)
    }
    ///Bit 4 - MPUP0RST
    #[inline(always)]
    pub fn mpup0rst(&mut self) -> MPUP0RST_W<'_, MP_GRSTCSETRrs> {
        MPUP0RST_W::new(self, 4)
    }
    ///Bit 5 - MPUP1RST
    #[inline(always)]
    pub fn mpup1rst(&mut self) -> MPUP1RST_W<'_, MP_GRSTCSETRrs> {
        MPUP1RST_W::new(self, 5)
    }
}
/**This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.

You can [`read`](crate::Reg::read) this register and get [`mp_grstcsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_grstcsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_GRSTCSETR)*/
pub struct MP_GRSTCSETRrs;
impl crate::RegisterSpec for MP_GRSTCSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_grstcsetr::R`](R) reader structure
impl crate::Readable for MP_GRSTCSETRrs {}
///`write(|w| ..)` method takes [`mp_grstcsetr::W`](W) writer structure
impl crate::Writable for MP_GRSTCSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_GRSTCSETR to value 0
impl crate::Resettable for MP_GRSTCSETRrs {}
