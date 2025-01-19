///Register `OTFDEC_R3KEYR1` writer
pub type W = crate::W<OTFDEC_R3KEYR1rs>;
/**Field `REGx_KEY` writer - Region key, bits \[63:32\]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\]
bitfield.*/
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OTFDEC_R3KEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /**Bits 0:31 - Region key, bits \[63:32\]
    Refer to the OTFDEC_RxKEYR0 register for description of the KEY\[127:0\]
    bitfield.*/
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<OTFDEC_R3KEYR1rs> {
        REGX_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 3 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r3keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#OTFDEC1:OTFDEC_R3KEYR1)*/
pub struct OTFDEC_R3KEYR1rs;
impl crate::RegisterSpec for OTFDEC_R3KEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`otfdec_r3keyr1::W`](W) writer structure
impl crate::Writable for OTFDEC_R3KEYR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTFDEC_R3KEYR1 to value 0
impl crate::Resettable for OTFDEC_R3KEYR1rs {
    const RESET_VALUE: u32 = 0;
}
