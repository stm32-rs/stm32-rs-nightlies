///Register `EXTICR3` reader
pub type R = crate::R<EXTICR3rs>;
///Register `EXTICR3` writer
pub type W = crate::W<EXTICR3rs>;
///Field `PC_EXTI8` reader - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI8_R = crate::FieldReader;
///Field `PC_EXTI9` reader - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI9_R = crate::FieldReader;
///Field `PC_EXTI10` reader - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI10_R = crate::FieldReader;
///Field `PC_EXTI11` reader - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI11_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti8(&self) -> PC_EXTI8_R {
        PC_EXTI8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti9(&self) -> PC_EXTI9_R {
        PC_EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti10(&self) -> PC_EXTI10_R {
        PC_EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port configuration EXTI {2 * 4 + i} This bitfield selects the source input to the EXTI input {2 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti11(&self) -> PC_EXTI11_R {
        PC_EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR3")
            .field("pc_exti8", &self.pc_exti8())
            .field("pc_exti9", &self.pc_exti9())
            .field("pc_exti10", &self.pc_exti10())
            .field("pc_exti11", &self.pc_exti11())
            .finish()
    }
}
impl W {}
/**SBS external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:EXTICR3)*/
pub struct EXTICR3rs;
impl crate::RegisterSpec for EXTICR3rs {
    type Ux = u32;
}
///`read()` method returns [`exticr3::R`](R) reader structure
impl crate::Readable for EXTICR3rs {}
///`write(|w| ..)` method takes [`exticr3::W`](W) writer structure
impl crate::Writable for EXTICR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR3 to value 0
impl crate::Resettable for EXTICR3rs {}
