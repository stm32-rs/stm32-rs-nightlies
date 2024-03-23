#[doc = "Register `APB1H_FZ` reader"]
pub type R = crate::R<APB1H_FZrs>;
#[doc = "Register `APB1H_FZ` writer"]
pub type W = crate::W<APB1H_FZrs>;
#[doc = "Field `DBG_I2C4_STOP` reader - DBG_I2C4_STOP"]
pub type DBG_I2C4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C4_STOP` writer - DBG_I2C4_STOP"]
pub type DBG_I2C4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DBG_I2C4_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<APB1H_FZrs> {
        DBG_I2C4_STOP_W::new(self, 1)
    }
}
#[doc = "APB Low Freeze Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1h_fz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1h_fz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1H_FZrs;
impl crate::RegisterSpec for APB1H_FZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1h_fz::R`](R) reader structure"]
impl crate::Readable for APB1H_FZrs {}
#[doc = "`write(|w| ..)` method takes [`apb1h_fz::W`](W) writer structure"]
impl crate::Writable for APB1H_FZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1H_FZ to value 0"]
impl crate::Resettable for APB1H_FZrs {
    const RESET_VALUE: u32 = 0;
}
