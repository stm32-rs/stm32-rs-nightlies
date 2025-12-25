///Register `MUX` reader
pub type R = crate::R<MUXrs>;
///Register `MUX` writer
pub type W = crate::W<MUXrs>;
///Field `MUX0` reader - Select the HDPy output among the 16 available signals
pub type MUX0_R = crate::FieldReader;
///Field `MUX0` writer - Select the HDPy output among the 16 available signals
pub type MUX0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MUX1` reader - Select the HDPy output among the 16 available signals
pub type MUX1_R = crate::FieldReader;
///Field `MUX1` writer - Select the HDPy output among the 16 available signals
pub type MUX1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MUX2` reader - Select the HDPy output among the 16 available signals
pub type MUX2_R = crate::FieldReader;
///Field `MUX2` writer - Select the HDPy output among the 16 available signals
pub type MUX2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MUX3` reader - Select the HDPy output among the 16 available signals
pub type MUX3_R = crate::FieldReader;
///Field `MUX3` writer - Select the HDPy output among the 16 available signals
pub type MUX3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MUX4` reader - Select the HDPy output among the 16 available signals
pub type MUX4_R = crate::FieldReader;
///Field `MUX4` writer - Select the HDPy output among the 16 available signals
pub type MUX4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MUX5` reader - Select the HDPy output among the 16 available signals
pub type MUX5_R = crate::FieldReader;
///Field `MUX5` writer - Select the HDPy output among the 16 available signals
pub type MUX5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MUX6` reader - Select the HDPy output among the 16 available signals
pub type MUX6_R = crate::FieldReader;
///Field `MUX6` writer - Select the HDPy output among the 16 available signals
pub type MUX6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MUX7` reader - Select the HDPy output among the 16 available signals
pub type MUX7_R = crate::FieldReader;
///Field `MUX7` writer - Select the HDPy output among the 16 available signals
pub type MUX7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux0(&self) -> MUX0_R {
        MUX0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux1(&self) -> MUX1_R {
        MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux2(&self) -> MUX2_R {
        MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux3(&self) -> MUX3_R {
        MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux4(&self) -> MUX4_R {
        MUX4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux5(&self) -> MUX5_R {
        MUX5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux6(&self) -> MUX6_R {
        MUX6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux7(&self) -> MUX7_R {
        MUX7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUX")
            .field("mux0", &self.mux0())
            .field("mux1", &self.mux1())
            .field("mux2", &self.mux2())
            .field("mux3", &self.mux3())
            .field("mux4", &self.mux4())
            .field("mux5", &self.mux5())
            .field("mux6", &self.mux6())
            .field("mux7", &self.mux7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux0(&mut self) -> MUX0_W<'_, MUXrs> {
        MUX0_W::new(self, 0)
    }
    ///Bits 4:7 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux1(&mut self) -> MUX1_W<'_, MUXrs> {
        MUX1_W::new(self, 4)
    }
    ///Bits 8:11 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux2(&mut self) -> MUX2_W<'_, MUXrs> {
        MUX2_W::new(self, 8)
    }
    ///Bits 12:15 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux3(&mut self) -> MUX3_W<'_, MUXrs> {
        MUX3_W::new(self, 12)
    }
    ///Bits 16:19 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux4(&mut self) -> MUX4_W<'_, MUXrs> {
        MUX4_W::new(self, 16)
    }
    ///Bits 20:23 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux5(&mut self) -> MUX5_W<'_, MUXrs> {
        MUX5_W::new(self, 20)
    }
    ///Bits 24:27 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux6(&mut self) -> MUX6_W<'_, MUXrs> {
        MUX6_W::new(self, 24)
    }
    ///Bits 28:31 - Select the HDPy output among the 16 available signals
    #[inline(always)]
    pub fn mux7(&mut self) -> MUX7_W<'_, MUXrs> {
        MUX7_W::new(self, 28)
    }
}
/**HDP multiplexer control register

You can [`read`](crate::Reg::read) this register and get [`mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HDP:MUX)*/
pub struct MUXrs;
impl crate::RegisterSpec for MUXrs {
    type Ux = u32;
}
///`read()` method returns [`mux::R`](R) reader structure
impl crate::Readable for MUXrs {}
///`write(|w| ..)` method takes [`mux::W`](W) writer structure
impl crate::Writable for MUXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MUX to value 0
impl crate::Resettable for MUXrs {}
